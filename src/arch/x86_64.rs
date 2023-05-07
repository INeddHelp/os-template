// Simple x86_64 implementation

#![no_std]

// GDT descriptor
#[repr(C)]
struct GdtDescriptor {
    limit_low: u16,
    base_low: u16,
    base_middle: u8,
    access: u8,
    granularity: u8,
    base_high: u8,
}

// GDT table
static GDT: [GdtDescriptor; 3] = [
    // Null descriptor
    GdtDescriptor {
        limit_low: 0,
        base_low: 0,
        base_middle: 0,
        access: 0,
        granularity: 0,
        base_high: 0,
    },
    // Code descriptor
    GdtDescriptor {
        limit_low: 0xFFFF,
        base_low: 0,
        base_middle: 0,
        access: 0x9A,
        granularity: 0xCF,
        base_high: 0,
    },
    // Data descriptor
    GdtDescriptor {
        limit_low: 0xFFFF,
        base_low: 0,
        base_middle: 0,
        access: 0x92,
        granularity: 0xCF,
        base_high: 0,
    },
];

#[no_mangle]
pub extern "C" fn kmain() -> ! {
    // Load the GDT
    unsafe {
        let descriptor = GdtDescriptorPointer {
            size: (core::mem::size_of::<[GdtDescriptor; 3]>() - 1) as u16,
            offset: &GDT as *const _ as u64,
        };
        asm!("lgdt [{0}]", in(reg) &descriptor);
    }

    // Enter an infinite loop
    loop {}
}
