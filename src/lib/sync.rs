use core::sync::atomic::{AtomicBool, Ordering};

pub struct Spinlock {
    lock: AtomicBool,
}

impl Spinlock {
    pub fn new() -> Spinlock {
        Spinlock {
            lock: AtomicBool::new(false),
        }
    }

    pub fn lock(&self) {
        while !self.lock.compare_and_swap(false, true, Ordering::Acquire) {}
    }

    pub fn unlock(&self) {
        self.lock.store(false, Ordering::Release);
    }
}
