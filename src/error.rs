//! Error types and result handling
//!
//! Defines all error types that can occur in the logging library.
//! Uses thiserror for ergonomic error handling.

use std::io;
use thiserror::Error;

/// Result type alias using LoglyError
pub type Result<T> = std::result::Result<T, LoglyError>;

/// Main error type for the logging library.
///
/// Covers all possible error conditions including I/O errors, configuration errors,
/// GPU errors, and runtime errors.
#[derive(Error, Debug)]
pub enum LoglyError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),

    #[error("Invalid log level: {0}")]
    InvalidLevel(String),

    #[error("Sink not found: {0}")]
    SinkNotFound(usize),

    #[error("Invalid format string: {0}")]
    InvalidFormat(String),

    #[error("Regex error: {0}")]
    Regex(#[from] regex::Error),

    #[error("Channel send error")]
    ChannelSend,

    #[error("Logger already initialized")]
    AlreadyInitialized,

    #[error("GPU/CUDA error: {0}")]
    GpuError(String),

    #[error("Callback execution error: {0}")]
    CallbackError(String),

    #[error("Version check error: {0}")]
    VersionCheckError(String),

    #[error("Custom level already exists: {0}")]
    CustomLevelExists(String),

    #[error("Exception: {0}\nBacktrace:\n{1}")]
    Exception(String, String),

    #[error("{0}")]
    Custom(String),
}

impl LoglyError {
    pub fn report_bug_message(&self) -> String {
        format!(
            "\n⚠️  An error occurred: {}\n\nIf you believe this is a bug in logly, please report it:\n  Rust crate: https://github.com/muhammad-fiaz/logly-rs/issues\n  Python package: https://github.com/muhammad-fiaz/logly/issues\n",
            self
        )
    }
}
