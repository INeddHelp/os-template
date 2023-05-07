#![no_std]

#[no_mangle]
pub extern "C" fn kmain() -> ! {
    // Set up some initial values
    let mut x = 0;
    let y = 1;

    // Enter an infinite loop
    loop {
        x += y;
    }
}
