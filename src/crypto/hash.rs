use std::convert::TryInto;

// TODO: Define hash types and their constants

pub trait Hash {
    fn block_size(&self) -> usize;
    fn output_size(&self) -> usize;

    // TODO: Define hash methods
}

pub struct Sha256 {
    // TODO: Define internal state variables
}

impl Sha256 {
    pub fn new() -> Self {
        // TODO: Initialize internal state variables
        Self {}
    }
}

impl Hash for Sha256 {
    fn block_size(&self) -> usize {
        64
    }

    fn output_size(&self) -> usize {
        32
    }

    // TODO: Implement hash methods
}

// TODO: Implement other hash types
