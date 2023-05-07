// Define system call numbers
pub const SYSCALL_EXIT: usize = 93;
pub const SYSCALL_WRITE: usize = 64;
pub const SYSCALL_READ: usize = 63;

// Define the system call handler function
pub fn syscall_handler(
    syscall_number: usize,
    arg1: usize,
    arg2: usize,
    arg3: usize,
) -> usize {
    match syscall_number {
        SYSCALL_EXIT => {
            // Exit the current process
            process::exit(arg1 as i32);
            0
        }
        SYSCALL_WRITE => {
            // Write data to a file descriptor
            let fd = arg1 as i32;
            let buffer = arg2 as *const u8;
            let size = arg3 as usize;
            match file::file_descriptor_write(fd, buffer, size) {
                Ok(bytes_written) => bytes_written as usize,
                Err(_) => 0,
            }
        }
        SYSCALL_READ => {
            // Read data from a file descriptor
            let fd = arg1 as i32;
            let buffer = arg2 as *mut u8;
            let size = arg3 as usize;
            match file::file_descriptor_read(fd, buffer, size) {
                Ok(bytes_read) => bytes_read as usize,
                Err(_) => 0,
            }
        }
        _ => {
            // Unknown system call number
            0
        }
    }
}
