use core::ptr;

// Thread ID type
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ThreadId(pub usize);

// Thread structure
pub struct Thread {
    id: ThreadId,
    stack: *mut u8,
    state: ThreadState,
}

impl Thread {
    // Creates a new thread with the given function as its entry point
    pub fn new(entry: fn()) -> Self {
        // Allocate stack space for the thread
        let stack_size = 4096; // 4 KB
        let stack = unsafe {
            let stack = alloc(stack_size);
            if stack.is_null() {
                panic!("Failed to allocate stack for new thread");
            }
            stack.add(stack_size)
        };

        // Initialize the stack with the thread entry point
        unsafe {
            ptr::write_volatile(stack.offset(-4) as *mut fn(), entry);
        }

        // Create and return the thread
        Self {
            id: ThreadId(0), // TODO: generate unique thread ID
            stack,
            state: ThreadState::Ready,
        }
    }

    // Returns the thread ID
    pub fn id(&self) -> ThreadId {
        self.id
    }

    // Sets the thread state
    pub fn set_state(&mut self, state: ThreadState) {
        self.state = state;
    }

    // Returns the thread state
    pub fn state(&self) -> ThreadState {
        self.state
    }

    // Runs the thread, starting at its entry point
    pub fn run(&mut self) {
        // Set the current thread
        unsafe {
            CURRENT_THREAD = self;
        }

        // Call the thread entry point
        let entry: fn() = unsafe { ptr::read_volatile(self.stack.offset(-4) as *const fn()) };
        entry();

        // Thread has exited
        self.set_state(ThreadState::Exited);
        unsafe {
            yield_cpu(); // Switch to next runnable thread
        }
    }
}

// Thread state
#[derive(Debug, PartialEq, Eq)]
pub enum ThreadState {
    Ready,
    Running,
    Blocked,
    Exited,
}

// Pointer to the currently running thread
static mut CURRENT_THREAD: *mut Thread = ptr::null_mut();

// Switches to the next runnable thread
fn yield_cpu() {
    // TODO: implement thread scheduling
}

// Allocates a block of memory
fn alloc(size: usize) -> *mut u8 {
    // TODO: implement memory allocation
    ptr::null_mut()
}
