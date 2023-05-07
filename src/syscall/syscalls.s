// write system call implementation
.global _write
_write:
    mov %rax, %rdi       // move file descriptor into rdi
    mov %rsi, %rsi       // move string buffer into rsi
    mov %rdx, %rdx       // move buffer length into rdx
    mov $1, %rax         // move system call number for write into rax
    syscall              // execute system call
    ret                  // return from function
