use std::io::{Read, Write};
use std::fs::{File, OpenOptions};
use std::path::Path;
use crate::storage::Error;

pub const BLOCK_SIZE: usize = 4096;

pub struct Block {
    id: u64,
    data: Vec<u8>,
}

impl Block {
    pub fn new(id: u64) -> Self {
        Self {
            id,
            data: vec![0; BLOCK_SIZE],
        }
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn read(&mut self, file: &mut File) -> Result<(), Error> {
        let offset = self.id * BLOCK_SIZE as u64;
        file.seek(std::io::SeekFrom::Start(offset))?;
        file.read_exact(&mut self.data)?;
        Ok(())
    }

    pub fn write(&self, file: &mut File) -> Result<(), Error> {
        let offset = self.id * BLOCK_SIZE as u64;
        file.seek(std::io::SeekFrom::Start(offset))?;
        file.write_all(&self.data)?;
        Ok(())
    }
}

pub struct BlockDevice {
    file: File,
    num_blocks: u64,
}

impl BlockDevice {
    pub fn new<P: AsRef<Path>>(path: P, num_blocks: u64) -> Result<Self, Error> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path)?;
        let mut bd = BlockDevice {
            file,
            num_blocks,
        };
        bd.init()?;
        Ok(bd)
    }

    fn init(&mut self) -> Result<(), Error> {
        let size = self.file.metadata()?.len();
        let expected_size = self.num_blocks * BLOCK_SIZE as u64;
        if size < expected_size {
            self.file.set_len(expected_size)?;
        }
        Ok(())
    }

    pub fn get_block(&mut self, id: u64) -> Result<Block, Error> {
        if id >= self.num_blocks {
            return Err(Error::InvalidBlockId);
        }
        let mut block = Block::new(id);
        block.read(&mut self.file)?;
        Ok(block)
    }

    pub fn write_block(&mut self, block: &Block) -> Result<(), Error> {
        if block.id() >= self.num_blocks {
            return Err(Error::InvalidBlockId);
        }
        block.write(&mut self.file)?;
        Ok(())
    }
}

// TODO: Implement the `Drop` trait for `BlockDevice`.