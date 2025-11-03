//! Log record structure
//!
//! Defines the core LogRecord type that represents a single log entry.
//! Records contain timestamp, level, message, location info, and custom fields.

use crate::level::Level;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A single log record containing all information about a log entry.
///
/// Records are created by the logger and passed through filters, formatters,
/// and callbacks before being written to sinks.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogRecord {
    /// UTC timestamp when the log was created
    pub timestamp: DateTime<Utc>,
    /// Log level (TRACE, DEBUG, INFO, etc.)
    pub level: Level,
    /// Log message text
    pub message: String,
    /// Module name where the log originated
    pub module: Option<String>,
    /// Function name where the log originated
    pub function: Option<String>,
    /// Source file name
    pub filename: Option<String>,
    /// Line number in source file
    pub lineno: Option<u32>,
    /// Additional structured fields (key-value pairs)
    pub fields: HashMap<String, serde_json::Value>,
}

impl LogRecord {
    /// Creates a new log record with the specified level and message.
    ///
    /// # Arguments
    ///
    /// * `level` - Log level
    /// * `message` - Log message text
    ///
    /// # Returns
    ///
    /// A new LogRecord with current timestamp and empty location/fields
    pub fn new(level: Level, message: String) -> Self {
        Self {
            timestamp: Utc::now(),
            level,
            message,
            module: None,
            function: None,
            filename: None,
            lineno: None,
            fields: HashMap::new(),
        }
    }

    /// Adds a custom field to the log record.
    ///
    /// # Arguments
    ///
    /// * `key` - Field name
    /// * `value` - Field value (any JSON-serializable value)
    ///
    /// # Returns
    ///
    /// Self for method chaining
    pub fn with_field(mut self, key: String, value: serde_json::Value) -> Self {
        self.fields.insert(key, value);
        self
    }

    /// Sets location information for the log record.
    ///
    /// # Arguments
    ///
    /// * `module` - Module name
    /// * `function` - Function name
    /// * `filename` - Source file name
    /// * `lineno` - Line number
    ///
    /// # Returns
    ///
    /// Self for method chaining
    pub fn with_location(
        mut self,
        module: Option<String>,
        function: Option<String>,
        filename: Option<String>,
        lineno: Option<u32>,
    ) -> Self {
        self.module = module;
        self.function = function;
        self.filename = filename;
        self.lineno = lineno;
        self
    }
}
