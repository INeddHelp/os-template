/// A custom error type for the operating system
#[derive(Debug, Clone, PartialEq)]
pub enum OsError {
    InvalidArgument,
    OutOfMemory,
    PermissionDenied,
    FileNotFound,
    IOError,
    NotImplemented,
    Other(String),
}

impl OsError {
    /// Create a new `OsError` instance with a custom message
    pub fn new(message: &str) -> Self {
        OsError::Other(message.to_owned())
    }
}

/// A convenient type alias for the `Result` type with the `OsError` error type
pub type OsResult<T> = Result<T, OsError>;