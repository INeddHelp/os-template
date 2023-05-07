#[derive(Debug)]
pub struct MemoryManager {
    start: usize,
    end: usize,
    next: usize,
}

impl MemoryManager {
    pub fn new(start: usize, end: usize) -> MemoryManager {
        MemoryManager {
            start,
            end,
            next: start,
        }
    }

    pub fn allocate(&mut self, size: usize, align: usize) -> Option<usize> {
        let aligned_next = align_up(self.next, align);
        let new_next = aligned_next + size;

        if new_next <= self.end {
            let allocation = aligned_next;
            self.next = new_next;
            Some(allocation)
        } else {
            None
        }
    }
}

fn align_up(addr: usize, align: usize) -> usize {
    (addr + align - 1) & !(align - 1)
}
