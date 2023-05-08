// TODO: Implement the DNS module

use std::net::{Ipv4Addr, SocketAddr};

struct DnsQuery {
    qname: String,
    qtype: u16,
    qclass: u16,
}

struct DnsResponse {
    id: u16,
    qr: u16,
    opcode: u16,
    aa: u16,
    tc: u16,
    rd: u16,
    ra: u16,
    z: u16,
    rcode: u16,
    qdcount: u16,
    ancount: u16,
    nscount: u16,
    arcount: u16,
    answers: Vec<DnsRecord>,
}

struct DnsRecord {
    name: String,
    rtype: u16,
    rclass: u16,
    ttl: u32,
    rdlength: u16,
    rdata: Vec<u8>,
}

fn resolve(hostname: &str) -> Result<Ipv4Addr, String> {
    let query = DnsQuery {
        qname: hostname.to_string(),
        qtype: 1, // A record type
        qclass: 1, // IN (Internet) class
    };

    let socket_addr: SocketAddr = "8.8.8.8:53".parse().unwrap();
    let socket = std::net::UdpSocket::bind("0.0.0.0:0").unwrap();
    let mut request_buf = Vec::new();
    // TODO: Implement the DNS message encoding and decoding
    let request = encode_dns_query(&query, &mut request_buf);
    let _ = socket.send_to(&request, &socket_addr).unwrap();

    let mut response_buf = [0; 512];
    let (_size, _src) = socket.recv_from(&mut response_buf).unwrap();
    let response = decode_dns_response(&response_buf);

    match response.answers.into_iter().find(|record| record.rtype == 1) {
        Some(record) => {
            let octets = &record.rdata[..];
            if octets.len() == 4 {
                let octet_array: [u8; 4] = [
                    octets[0], octets[1], octets[2], octets[3],
                ];
                Ok(Ipv4Addr::from(octet_array))
            } else {
                Err(format!("Invalid A record data length: {}", octets.len()))
            }
        }
        None => Err(format!("No A record found for {}", hostname)),
    }
}

fn encode_dns_query(query: &DnsQuery, buf: &mut Vec<u8>) -> &[u8] {
    // TODO: Implement DNS query encoding
    buf
}

fn decode_dns_response(buf: &[u8]) -> DnsResponse {
    // TODO: Implement DNS response decoding
    DnsResponse {
        id: 0,
        qr: 0,
        opcode: 0,
        aa: 0,
        tc: 0,
        rd: 0,
        ra: 0,
        z: 0,
        rcode: 0,
        qdcount: 0,
        ancount: 0,
        nscount: 0,
        arcount: 0,
        answers: Vec::new(),
    }
}
