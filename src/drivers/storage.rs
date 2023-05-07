// Storage driver implementation for x86_64 architecture
// Requires a compatible storage device

use x86_64::instructions::port::{Port, PortWriteOnly};

// Storage device constants
const STORAGE_DATA_PORT: u16 = 0x1F0;
const STORAGE_COMMAND_PORT: u16 = 0x1F7;

// Storage driver struct
pub struct Storage {
    data_port: Port<u8>,
    command_port: PortWriteOnly<u8>,
}

impl Storage {
    // Initialize the storage driver
    pub fn new() -> Storage {
        Storage {
            data_port: Port::new(STORAGE_DATA_PORT),
            command_port: PortWriteOnly::new(STORAGE_COMMAND_PORT),
        }
    }

    // Read data from the storage device
    pub fn read(&mut self, buffer: &mut [u8]) {
        for sector in buffer.chunks_exact_mut(512) {
            self.read_sector(sector);
        }
    }

    // Write data to the storage device
    pub fn write(&mut self, buffer: &[u8]) {
        for sector in buffer.chunks_exact(512) {
            self.write_sector(sector);
        }
    }

    // Read a sector from the storage device
    fn read_sector(&mut self, sector: &mut [u8]) {
        // Issue a read command to the storage device
        self.command_port.write(0x20);

        // Read the sector data
        for byte in sector.iter_mut() {
            *byte = self.data_port.read();
        }
    }

    // Write a sector to the storage device
    fn write_sector(&mut self, sector: &[u8]) {
        // Issue a write command to the storage device
        self.command_port.write(0x30);

        // Write the sector data
        for byte in sector.iter() {
            self.data_port.write(*byte);
        }
    }
}
