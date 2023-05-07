pub struct Vec<T> {
    buffer: Box<[T]>,
    len: usize,
}

impl<T> Vec<T> {
    pub fn new() -> Vec<T> {
        Vec {
            buffer: Box::new([]),
            len: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn push(&mut self, value: T) {
        if self.buffer.len() == self.len {
            let new_len = if self.len == 0 { 1 } else { self.len * 2 };
            let mut new_buffer = Vec::with_capacity(new_len);
            new_buffer.extend_from_slice(&self.buffer[..self.len]);
            self.buffer = new_buffer;
        }

        self.buffer[self.len] = value;
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            Some(std::mem::replace(&mut self.buffer[self.len], unsafe { std::mem::uninitialized() }))
        }
    }
}
