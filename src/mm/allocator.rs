use super::memory::{Frame, FrameAllocator};
use alloc::vec::Vec;
use buddy_system_allocator::LockedHeap;

const HEAP_START: usize = 0o_000_001_000_000_0000;
const HEAP_SIZE: usize = 100 * 1024;

static mut HEAP: [u8; HEAP_SIZE] = [0; HEAP_SIZE];
static mut ALLOCATOR: Option<buddy_system_allocator::Heap> = None;

pub struct Allocator;

unsafe impl FrameAllocator for Allocator {
    unsafe fn allocate_frame(&mut self) -> Option<Frame> {
        ALLOCATOR.as_mut().unwrap().allocate_frame().map(Frame::from_start_address)
    }

    unsafe fn deallocate_frame(&mut self, frame: Frame) {
        ALLOCATOR.as_mut().unwrap().deallocate_frame(frame.start_address());
    }
}

impl Allocator {
    pub fn init_heap() {
        unsafe {
            ALLOCATOR = Some(buddy_system_allocator::Heap::new(
                HEAP_START,
                HEAP_SIZE,
            ));
            LockedHeap::initialize(HEAP.as_ptr() as usize, HEAP_SIZE);
        }
    }
}
