#![no_std]

#[repr(align(4))]
#[repr(C)]
struct Context {
    gpr: [u32; 32],
    hi: u32,
    lo: u32,
    pc: u32,
}

#[no_mangle]
pub extern "C" fn entry() -> ! {
    // main entry point for the kernel
    loop {}
}

#[no_mangle]
pub extern "C" fn exception_handler(context: &mut Context) -> ! {
    // handle an exception
    loop {}
}

#[no_mangle]
pub extern "C" fn interrupt_handler(context: &mut Context) -> ! {
    // handle an interrupt
    loop {}
}
