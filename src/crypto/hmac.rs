use crypto::hash::{Hash, SHA256};
use std::ops::Deref;

pub struct Hmac<'a> {
    key: &'a [u8],
    hash: SHA256,
}

impl<'a> Hmac<'a> {
    pub fn new(key: &'a [u8]) -> Hmac<'a> {
        Hmac {
            key,
            hash: SHA256::new(),
        }
    }

    pub fn update(&mut self, data: &[u8]) {
        // TODO: Implement HMAC update function
        unimplemented!();
    }

    pub fn finalize(&self) -> Vec<u8> {
        // TODO: Implement HMAC finalization function
        unimplemented!();
    }
}

impl<'a> Deref for Hmac<'a> {
    type Target = SHA256;

    fn deref(&self) -> &SHA256 {
        &self.hash
    }
}
