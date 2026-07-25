//! One error type for everything that aborts a command with exit code 2.
//!
//! Check failures are *not* errors — they are results (exit code 1). This type
//! is for usage mistakes, unreadable content and I/O problems.

use std::fmt;

#[derive(Debug)]
pub struct AppError {
    pub message: String,
}

impl AppError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for AppError {}

pub type Result<T> = std::result::Result<T, AppError>;
