use core::time::Duration;

/// Get the current system time.
pub fn current_time() -> Duration {
    // implementation-specific code to get the current system time
    // ...
}

/// Measure the time it takes to execute a closure.
pub fn measure_time<F>(f: F) -> Duration
    where
        F: FnOnce(),
{
    let start_time = current_time();
    f();
    current_time() - start_time
}
