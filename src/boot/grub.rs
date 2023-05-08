use crate::boot::BootInfo;

pub fn init(boot_info: &BootInfo) {
    // TODO: Grub-specific initialization code
    println!("Grub initialized with boot information: {:?}", boot_info);
}
