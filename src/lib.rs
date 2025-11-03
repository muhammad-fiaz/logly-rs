//! Logly: High-performance structured logging library for Rust
//!
//! A production-ready logging library with async support, GPU acceleration,
//! file rotation, filtering, callbacks, and comprehensive error handling.
//!
//! # Features
//!
//! - **8 Log Levels**: TRACE, DEBUG, INFO, SUCCESS, WARNING, ERROR, FAIL, CRITICAL
//! - **Custom Log Levels**: Define your own levels with custom priorities and colors
//! - **Structured Logging**: JSON and custom format support
//! - **Async Logging**: Non-blocking writes with configurable buffers
//! - **GPU Acceleration**: Optional CUDA support for high-throughput scenarios
//! - **File Rotation**: Time-based and size-based rotation with retention policies
//! - **Filtering**: Level, module, and function-based filtering
//! - **Callbacks**: Log, color, and exception callbacks
//! - **Context Binding**: Persistent and temporary context fields
//!
//! # Quick Start
//!
//! ```no_run
//! use logly::prelude::*;
//!
//! let logger = Logger::new();
//! logger.add_sink(SinkConfig::default())?;
//!
//! logger.info("Application started".to_string())?;
//! logger.success("Operation completed!".to_string())?;
//! logger.warning("Warning message".to_string())?;
//! logger.error("Error occurred".to_string())?;
//! # Ok::<(), logly::LoglyError>(())
//! ```
//!
//! # Version
//!
//! Current version: 0.0.4
//!
//! # Repository
//!
//! https://github.com/muhammad-fiaz/logly-rs
//!
//! # Author
//!
//! muhammad-fiaz <contact@muhammadfiaz.com>

pub mod callback;
pub mod config;
pub mod config_file;
pub mod error;
pub mod filter;
pub mod format;
pub mod gpu;
pub mod level;
pub mod logger;
pub mod record;
pub mod rotation;
pub mod sink;
pub mod utils;
pub mod version;

pub use callback::{CallbackManager, ColorCallback, ExceptionCallback, LogCallback};
pub use config::LoggerConfig;
pub use config_file::ConfigFileLoader;
pub use error::{LoglyError, Result};
pub use gpu::GpuLogger;
pub use level::{CustomLevel, Level};
pub use logger::Logger;
pub use record::LogRecord;
pub use rotation::{RotationManager, RotationPolicy};
pub use sink::{Sink, SinkConfig};
pub use version::VersionChecker;

// Re-export commonly used types
pub use chrono::{DateTime, Utc};
pub use serde_json::Value as JsonValue;

// Prelude for convenient imports
pub mod prelude {
    pub use crate::callback::{CallbackManager, ColorCallback, ExceptionCallback, LogCallback};
    pub use crate::config::LoggerConfig;
    pub use crate::config_file::ConfigFileLoader;
    pub use crate::gpu::GpuLogger;
    pub use crate::level::{CustomLevel, Level};
    pub use crate::logger::Logger;
    pub use crate::record::LogRecord;
    pub use crate::rotation::{RotationManager, RotationPolicy};
    pub use crate::sink::{Sink, SinkConfig};
    pub use crate::version::VersionChecker;
}
