use core::fmt::{self, Write};
use spin::Mutex;

// An enum to represent the different levels of log messages
enum LogLevel {
    Info,
    Warning,
    Error,
}

// A struct to hold information about each log message
struct LogMessage {
    level: LogLevel,
    message: &'static str,
}

// A simple implementation of a logger
pub struct Logger {
    messages: Mutex<Vec<LogMessage>>,
}

impl Logger {
    pub const fn new() -> Self {
        Self {
            messages: Mutex::new(Vec::new()),
        }
    }

    pub fn log_info(&self, message: &'static str) {
        self.log(LogLevel::Info, message);
    }

    pub fn log_warning(&self, message: &'static str) {
        self.log(LogLevel::Warning, message);
    }

    pub fn log_error(&self, message: &'static str) {
        self.log(LogLevel::Error, message);
    }

    fn log(&self, level: LogLevel, message: &'static str) {
        let mut messages = self.messages.lock();
        messages.push(LogMessage { level, message });
    }
}

// Implementing the `Write` trait to allow writing formatted strings to the logger
impl Write for Logger {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.log_info(s);
        Ok(())
    }
}
