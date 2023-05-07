use crate::ip::IpAddr;
use crate::io::Socket;
use crate::sync::WaitQueue;
use core::cell::UnsafeCell;

// Constants defining the protocol and various fields within the UDP header
const UDP_PROTOCOL: u8 = 17;
const UDP_SRC_PORT_OFFSET: usize = 0;
const UDP_DST_PORT_OFFSET: usize = 2;
const UDP_LENGTH_OFFSET: usize = 4;
const UDP_CHECKSUM_OFFSET: usize = 6;
const UDP_HEADER_LENGTH: usize = 8;

// UDP packet structure
#[repr(packed)]
struct UdpPacket {
    src_port: u16,
    dst_port: u16,
    length: u16,
    checksum: u16,
    data: [u8; 0],
}

impl UdpPacket {
    // Returns the total length of the UDP packet
    fn len(&self) -> usize {
        (usize::from(self.length)).to_be_bytes().len() + UDP_HEADER_LENGTH
    }

    // Returns a pointer to the data field in the UDP packet
    fn data(&self) -> *const u8 {
        &self.data as *const u8
    }
}

// UDP socket structure
pub struct UdpSocket {
    // Underlying IP socket
    socket: Socket,

    // Wait queue for blocking on incoming packets
    rx_queue: UnsafeCell<WaitQueue>,

    // Buffer for incoming packets
    rx_buffer: UnsafeCell<Option<(IpAddr, Vec<u8>)>>,
}

impl UdpSocket {
    // Creates a new UDP socket and binds it to the specified IP address and port
    pub fn new(addr: IpAddr, port: u16) -> Self {
        let socket = Socket::new(addr, UDP_PROTOCOL, port);
        let rx_queue = UnsafeCell::new(WaitQueue::new());
        let rx_buffer = UnsafeCell::new(None);

        Self {
            socket,
            rx_queue,
            rx_buffer,
        }
    }

    // Receives a UDP packet, blocking until one is available
    pub fn recv(&self) -> (IpAddr, Vec<u8>) {
        // Block until a packet is available
        let mut rx_queue = unsafe { &mut *self.rx_queue.get() };
        rx_queue.wait();

        // Retrieve the incoming packet and clear the receive buffer
        let rx_buffer = unsafe { &mut *self.rx_buffer.get() };
        let (src_addr, data) = rx_buffer.take().unwrap();
        data.into_iter().enumerate().for_each(|(i, &byte)| {
            unsafe {
                *(((self as *const _) as *mut u8).add(UDP_HEADER_LENGTH + i)) = byte;
            }
        });

        (src_addr, data)
    }

    // Sends a UDP packet to the specified destination IP address and port
    pub fn send(&self, dst_addr: IpAddr, dst_port: u16, data: &[u8]) {
        // Construct the UDP header
        let mut packet = UdpPacket {
            src_port: self.socket.port(),
            dst_port,
            length: ((data.len() + UDP_HEADER_LENGTH) as u16).to_be(),
            checksum: 0,
            data: [0; 0],
        };

        // Copy the data into the packet buffer
        for (i, &byte) in data.iter().enumerate() {
            packet.data[i] = byte;
        }

        // Calculate the checksum
        packet.checksum = !checksum(&packet, self.socket.addr(), dst_addr, UDP_PROTOCOL, data.len());

        // Send the packet
        self
            .socket
            .send(dst_addr, UDP_PROTOCOL, &packet as *const _ as *const u8, packet.len());
    }

    // Handles an incoming UDP packet
    fn handle_packet(&self, src_addr: IpAddr, packet: &[u8]) {
        // Extract the UDP header fields
        let src_port = u16::from_be_bytes([packet[UDP_SRC_PORT_OFFSET], packet[UDP_SRC_PORT_OFFSET + 1]]);
        let dst_port = u16::from_be_bytes([packet[UDP_DST_PORT_OFFSET], packet[UDP_DST_PORT_OFFSET + 1]]);
        let length = u16::from_be_bytes([packet[UDP_LENGTH_OFFSET], packet[UDP_LENGTH_OFFSET + 1]]);

        // Verify that the destination port matches our socket's port
        if dst_port != self.socket.port() {
            return;
        }

        // Construct a buffer containing the packet data
        let mut data = vec![0; usize::from(length) - UDP_HEADER_LENGTH];
        data.iter_mut().enumerate().for_each(|(i, byte)| {
            *byte = packet[UDP_HEADER_LENGTH + i];
        });

        // Store the incoming packet in the receive buffer
        let rx_buffer = unsafe { &mut *self.rx_buffer.get() };
        *rx_buffer = Some((src_addr, data));

        // Unblock any threads waiting to receive data
        let mut rx_queue = unsafe { &mut *self.rx_queue.get() };
        rx_queue.notify_one();
    }
}

// Calculates the checksum for a UDP packet
fn checksum(packet: &UdpPacket, src_addr: IpAddr, dst_addr: IpAddr, protocol: u8, data_len: usize) -> u16 {
    let pseudo_header = pseudo_header(src_addr, dst_addr, protocol, data_len);
    let mut sum = 0;
    // Add the UDP header to the checksum
    let udp_header = unsafe {
        core::slice::from_raw_parts(packet as *const _ as *const u8, UDP_HEADER_LENGTH)
    };
    sum = add_checksum(sum, udp_header);

// Add the pseudo-header to the checksum
    sum = add_checksum(sum, &pseudo_header);

    !sum
}

// Calculates the pseudo-header for a UDP packet checksum
fn pseudo_header(src_addr: IpAddr, dst_addr: IpAddr, protocol: u8, data_len: usize) -> [u8; 12] {
    let mut header = [0; 12];
    match (src_addr, dst_addr) {
        (IpAddr::V4(src), IpAddr::V4(dst)) => {
            let src_bytes = src.octets();
            let dst_bytes = dst.octets();
            header[..4].copy_from_slice(&src_bytes);
            header[4..8].copy_from_slice(&dst_bytes);
        }
        (IpAddr::V6(src), IpAddr::V6(dst)) => {
            let src_bytes = src.octets();
            let dst_bytes = dst.octets();
            header[..16].copy_from_slice(&src_bytes);
            header[16..].copy_from_slice(&dst_bytes);
        }
        _ => unreachable!(),
    }
    header[9] = protocol;
    header[10..].copy_from_slice(&(data_len as u16).to_be_bytes());
    header
}

// Adds the checksum of a data slice to the current checksum
fn add_checksum(sum: u16, data: &[u8]) -> u16 {
    let mut sum = sum;
    let mut i = 0;
    while i < data.len() {
        let word = if i + 1 < data.len() {
            u16::from_be_bytes([data[i], data[i + 1]])
        } else {
            u16::from_be_bytes([data[i], 0x00])
        };
        sum = sum.wrapping_add(word);
        i += 2;
    }
    sum
}

// Calculates the one's complement of a 16-bit integer
fn ones_complement(mut sum: u16) -> u16 {
    while sum > 0xFFFF {
        sum = (sum >> 16) + (sum & 0xFFFF);
    }
    !sum
}

// Calculates the UDP checksum for a given packet
fn checksum(packet: &UdpPacket, src_addr: IpAddr, dst_addr: IpAddr, protocol: u8, data_len: usize) -> u16 {
    // Pseudo-header used for checksum calculation
    #[repr(packed)]
    struct PseudoHeader {
        src_addr: u32,
        dst_addr: u32,
        zero: u8,
        protocol: u8,
        length: u16,
    }

    // Construct the pseudo-header and UDP packet data
    let pseudo_header = PseudoHeader {
        src_addr: u32::from(src_addr),
        dst_addr: u32::from(dst_addr),
        zero: 0,
        protocol,
        length: ((data_len + UDP_HEADER_LENGTH) as u16).to_be(),
    };
    let mut data = Vec::with_capacity(packet.len());
    unsafe {
        let packet_ptr = packet as *const _ as *const u8;
        for i in 0..packet.len() {
            data.push(*packet_ptr.add(i));
        }
    }

// Calculate the checksum over the pseudo-header and packet data
    let mut sum = add_checksum(0, &pseudo_header as *const _ as *const u8);
    sum = add_checksum(sum, &data);

    ones_complement(sum)
}
