//! Callback system for log events
//!
//! This module provides a flexible callback system that allows users to hook into
//! various logging events. Callbacks are executed asynchronously and can be used for
//! monitoring, alerting, custom formatting, and error handling.

use crate::level::Level;
use crate::record::LogRecord;
use parking_lot::RwLock;
use std::sync::Arc;

/// Type alias for log callbacks that are executed for each log record.
/// Returns Ok(()) on success or Err(String) with error message on failure.
pub type LogCallback = Arc<dyn Fn(&LogRecord) -> Result<(), String> + Send + Sync>;

/// Type alias for color callbacks that customize color formatting.
/// Takes a Level and message text, returns formatted string with ANSI codes.
pub type ColorCallback = Arc<dyn Fn(Level, &str) -> String + Send + Sync>;

/// Type alias for exception callbacks that handle errors and exceptions.
/// Takes error message and backtrace string.
pub type ExceptionCallback = Arc<dyn Fn(&str, &str) + Send + Sync>;

/// Manages all callback types for the logging system.
///
/// CallbackManager is thread-safe and allows multiple callbacks of each type
/// to be registered and executed. All callbacks are stored in Arc<RwLock<>> for
/// safe concurrent access.
#[derive(Clone)]
pub struct CallbackManager {
    /// Collection of log callbacks executed for each log record
    log_callbacks: Arc<RwLock<Vec<LogCallback>>>,
    /// Collection of color callbacks for custom color formatting
    color_callbacks: Arc<RwLock<Vec<ColorCallback>>>,
    /// Collection of exception callbacks for error handling
    exception_callbacks: Arc<RwLock<Vec<ExceptionCallback>>>,
}

impl CallbackManager {
    /// Creates a new CallbackManager with empty callback collections.
    pub fn new() -> Self {
        Self {
            log_callbacks: Arc::new(RwLock::new(Vec::new())),
            color_callbacks: Arc::new(RwLock::new(Vec::new())),
            exception_callbacks: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Adds a log callback that will be executed for each log record.
    ///
    /// # Arguments
    /// * `callback` - Function that takes a LogRecord and returns Result<(), String>
    pub fn add_log_callback(&self, callback: LogCallback) {
        self.log_callbacks.write().push(callback);
    }

    /// Adds a color callback for custom color formatting.
    ///
    /// # Arguments
    /// * `callback` - Function that takes Level and message, returns formatted string
    pub fn add_color_callback(&self, callback: ColorCallback) {
        self.color_callbacks.write().push(callback);
    }

    /// Adds an exception callback for error handling.
    ///
    /// # Arguments
    /// * `callback` - Function that takes error message and backtrace
    pub fn add_exception_callback(&self, callback: ExceptionCallback) {
        self.exception_callbacks.write().push(callback);
    }

    /// Executes all registered log callbacks for a given record.
    ///
    /// # Arguments
    /// * `record` - The log record to pass to callbacks
    ///
    /// # Returns
    /// Vector of error messages from failed callbacks
    pub fn execute_log_callbacks(&self, record: &LogRecord) -> Vec<String> {
        let callbacks = self.log_callbacks.read();
        let mut errors = Vec::new();

        for callback in callbacks.iter() {
            if let Err(e) = callback(record) {
                errors.push(e);
            }
        }

        errors
    }

    /// Executes the first registered color callback.
    ///
    /// # Arguments
    /// * `level` - Log level for color selection
    /// * `message` - Message text to format
    ///
    /// # Returns
    /// Some(formatted_string) if callback exists, None otherwise
    pub fn execute_color_callbacks(&self, level: Level, message: &str) -> Option<String> {
        let callbacks = self.color_callbacks.read();
        callbacks.first().map(|callback| callback(level, message))
    }

    /// Executes all registered exception callbacks.
    ///
    /// # Arguments
    /// * `error` - Error message
    /// * `backtrace` - Stack backtrace string
    pub fn execute_exception_callbacks(&self, error: &str, backtrace: &str) {
        let callbacks = self.exception_callbacks.read();

        for callback in callbacks.iter() {
            callback(error, backtrace);
        }
    }

    /// Clears all registered log callbacks.
    pub fn clear_log_callbacks(&self) {
        self.log_callbacks.write().clear();
    }

    /// Clears all registered color callbacks.
    pub fn clear_color_callbacks(&self) {
        self.color_callbacks.write().clear();
    }

    /// Clears all registered exception callbacks.
    pub fn clear_exception_callbacks(&self) {
        self.exception_callbacks.write().clear();
    }

    /// Clears all callbacks of all types.
    pub fn clear_all(&self) {
        self.clear_log_callbacks();
        self.clear_color_callbacks();
        self.clear_exception_callbacks();
    }
}

impl Default for CallbackManager {
    fn default() -> Self {
        Self::new()
    }
}
