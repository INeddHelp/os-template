use core::fmt::{self, Write};

pub struct Console;

impl Write for Console {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            self.write_byte(byte)?;
        }
        Ok(())
    }
}

impl Console {
    pub fn new() -> Console {
        Console {}
    }

    pub fn write_byte(&mut self, byte: u8) -> fmt::Result {
        // write byte to console
        Ok(())
    }

    pub fn read_byte(&mut self) -> u8 {
        // read byte from console
        0
    }
}
