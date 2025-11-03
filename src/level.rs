//! Log levels with priority-based ordering
//!
//! Defines 8 standard log levels with numeric priorities and support for custom levels.
//! Each level has a default ANSI color code for console output.

use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

/// Standard log levels with numeric priorities.
/// 
/// Levels are ordered by severity, with lower numbers being less severe.
/// Each level has a unique priority value used for filtering and comparison.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Level {
    /// Trace level (priority 5) - Most verbose, for detailed debugging
    Trace = 5,
    /// Debug level (priority 10) - Debugging information
    Debug = 10,
    /// Info level (priority 20) - General information
    Info = 20,
    /// Success level (priority 25) - Successful operations
    Success = 25,
    /// Warning level (priority 30) - Warning messages
    Warning = 30,
    /// Error level (priority 40) - Error conditions
    Error = 40,
    /// Fail level (priority 45) - Operation failures
    Fail = 45,
    /// Critical level (priority 50) - Critical errors
    Critical = 50,
}

impl PartialOrd for Level {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Level {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (*self as u8).cmp(&(*other as u8))
    }
}

impl Level {
    pub fn as_str(&self) -> &'static str {
        match self {
            Level::Trace => "TRACE",
            Level::Debug => "DEBUG",
            Level::Info => "INFO",
            Level::Success => "SUCCESS",
            Level::Warning => "WARNING",
            Level::Error => "ERROR",
            Level::Fail => "FAIL",
            Level::Critical => "CRITICAL",
        }
    }

    pub fn priority(&self) -> u8 {
        *self as u8
    }

    pub fn default_color(&self) -> &'static str {
        match self {
            Level::Trace => "36",      // Cyan
            Level::Debug => "34",      // Blue
            Level::Info => "37",       // White
            Level::Success => "32",    // Green
            Level::Warning => "33",    // Yellow
            Level::Error => "31",      // Red
            Level::Fail => "35",       // Magenta
            Level::Critical => "91",   // Bright Red
        }
    }

    pub fn all_levels() -> Vec<Level> {
        vec![
            Level::Trace,
            Level::Debug,
            Level::Info,
            Level::Success,
            Level::Warning,
            Level::Error,
            Level::Fail,
            Level::Critical,
        ]
    }

    pub fn from_priority(priority: u8) -> Option<Self> {
        match priority {
            5 => Some(Level::Trace),
            10 => Some(Level::Debug),
            20 => Some(Level::Info),
            25 => Some(Level::Success),
            30 => Some(Level::Warning),
            40 => Some(Level::Error),
            45 => Some(Level::Fail),
            50 => Some(Level::Critical),
            _ => None,
        }
    }
}

impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for Level {
    type Err = crate::error::LoglyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "TRACE" => Ok(Level::Trace),
            "DEBUG" => Ok(Level::Debug),
            "INFO" => Ok(Level::Info),
            "SUCCESS" => Ok(Level::Success),
            "WARNING" | "WARN" => Ok(Level::Warning),
            "ERROR" => Ok(Level::Error),
            "FAIL" => Ok(Level::Fail),
            "CRITICAL" | "CRIT" => Ok(Level::Critical),
            _ => Err(crate::error::LoglyError::InvalidLevel(s.to_string())),
        }
    }
}

/// Custom log level with user-defined priority and color.
/// 
/// Allows users to define their own log levels beyond the standard 8 levels.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CustomLevel {
    /// Name of the custom level
    pub name: String,
    /// Priority value (0-255)
    pub priority: u8,
    /// ANSI color code for console output
    pub color: String,
}

impl CustomLevel {
    pub fn new(name: String, priority: u8, color: String) -> Self {
        Self { name, priority, color }
    }
}
