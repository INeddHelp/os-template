// Keyboard driver implementation for x86_64 architecture
// Requires a PS/2 keyboard

use x86_64::instructions::port::Port;

// PS/2 keyboard constants
const PS2_DATA_PORT: u16 = 0x60;
const PS2_STATUS_PORT: u16 = 0x64;
const PS2_ACK: u8 = 0xFA;

// Keyboard driver struct
pub struct Keyboard {
    status_port: Port<u8>,
    data_port: Port<u8>,
}

impl Keyboard {
    // Initialize the keyboard driver
    pub fn new() -> Keyboard {
        Keyboard {
            status_port: Port::new(PS2_STATUS_PORT),
            data_port: Port::new(PS2_DATA_PORT),
        }
    }

    // Check if a key is currently pressed
    fn is_key_pressed(&mut self) -> bool {
        self.status_port.read() & 0x01 != 0
    }

    // Read a key press from the keyboard
    pub fn read_key(&mut self) -> Option<u8> {
        if self.is_key_pressed() {
            let scancode = self.data_port.read();

            // Send an ACK to the keyboard controller
            self.data_port.write(PS2_ACK);

            Some(scancode)
        } else {
            None
        }
    }
}
