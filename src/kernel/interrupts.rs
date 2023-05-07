// Architecture-specific code could go in the arch subfolder
#[cfg(target_arch = "x86_64")]
mod x86_64;

#[cfg(target_arch = "armv7")]
mod armv7;

// Define an interrupt handler struct
pub struct InterruptHandler {
    // Interrupt vector number
    vector: u8,
    // Pointer to the interrupt handler function
    handler_func: fn(),
}

// Implement methods for the interrupt handler struct
impl InterruptHandler {
    // Initialize a new interrupt handler
    pub fn new(vector: u8, handler_func: fn()) -> Self {
        InterruptHandler {
            vector,
            handler_func,
        }
    }

    // Register the interrupt handler
    pub fn register(&self) {
        // Architecture-specific code could go here to register the interrupt handler
    }
}

// Define interrupt handler functions
fn timer_interrupt_handler() {
    // Timer interrupt handling code goes here
}

fn keyboard_interrupt_handler() {
    // Keyboard interrupt handling code goes here
}

// Define interrupt handler constants
const TIMER_INTERRUPT_VECTOR: u8 = 32;
const KEYBOARD_INTERRUPT_VECTOR: u8 = 33;

// Initialize interrupt handlers
static TIMER_INTERRUPT_HANDLER: InterruptHandler = InterruptHandler::new(TIMER_INTERRUPT_VECTOR, timer_interrupt_handler);
static KEYBOARD_INTERRUPT_HANDLER: InterruptHandler = InterruptHandler::new(KEYBOARD_INTERRUPT_VECTOR, keyboard_interrupt_handler);

// Register interrupt handlers
pub fn register_interrupt_handlers() {
    TIMER_INTERRUPT_HANDLER.register();
    KEYBOARD_INTERRUPT_HANDLER.register();
}
