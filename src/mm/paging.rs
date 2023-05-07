use super::memory::{Frame, FrameAllocator};
use x86_64::{
    structures::paging::{
        FrameAllocator as FrameAllocatorTrait, Mapper, MapperAllSizes, Page, PageTable,
        PageTableFlags, PageTableIndex, PhysFrame, Size4KiB,
    },
    PhysAddr, VirtAddr,
};

pub const PAGE_SIZE: usize = 4096;

pub type PageTableImpl = PageTable<Size4KiB>;

pub struct PageTableManager {
    page_table: PageTableImpl,
}

impl PageTableManager {
    pub fn new() -> Self {
        let page_table = unsafe { PageTableImpl::new() };
        PageTableManager { page_table }
    }

    pub fn activate(&mut self) {
        unsafe {
            self.page_table.activate();
        }
    }

    pub fn map_to(
        &mut self,
        page: Page<Size4KiB>,
        frame: Frame<Size4KiB>,
        flags: PageTableFlags,
        allocator: &mut impl FrameAllocatorTrait<Size4KiB>,
    ) -> Result<(), &'static str> {
        let frame_phys_addr = PhysAddr::new(frame.start_address());
        let page_entry = self.page_table.entry(page.clone());

        if page_entry.is_unused() {
            let frame = PhysFrame::containing_address(frame_phys_addr);
            let flags = flags | PageTableFlags::PRESENT;
            let map_result = unsafe {
                self.page_table
                    .map_to(page, frame, flags, allocator)
                    .map_err(|_| "Failed to map page to frame")?
            };

            if let Some(cache) = map_result {
                cache.flush();
            }
            Ok(())
        } else {
            Err("Page already mapped")
        }
    }

    pub fn unmap(
        &mut self,
        page: Page<Size4KiB>,
        allocator: &mut impl FrameAllocatorTrait<Size4KiB>,
    ) -> Result<(), &'static str> {
        let page_entry = self.page_table.entry(page.clone());

        if !page_entry.is_unused() {
            let (_, frame, flush) = unsafe { page_entry.into_frame_unchecked() };
            let frame = frame.unwrap();
            let flags = PageTableFlags::empty();
            unsafe {
                self.page_table
                    .unmap(page, allocator)
                    .map_err(|_| "Failed to unmap page from frame")?;
                flush();
            }
            allocator.deallocate_frame(Frame::from_start_address(
                frame.start_address().as_u64() as usize,
            ));
            Ok(())
        } else {
            Err("Page not mapped")
        }
    }

    pub fn translate_addr(&mut self, addr: VirtAddr) -> Option<PhysAddr> {
        self.page_table.translate_addr(addr)
    }
}
