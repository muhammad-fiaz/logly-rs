//! Sink management for log outputs
//!
//! Sinks are output destinations for log records (console, files, etc.).
//! Each sink has its own configuration, filters, formatter, and optional rotation.
//!
//! # Features
//!
//! - **Multiple outputs**: Console and file sinks
//! - **Async writes**: Non-blocking file writes with buffering
//! - **Rotation**: Automatic file rotation by size or time
//! - **Filtering**: Per-sink level, module, and function filters
//! - **Formatting**: Custom format templates and JSON output
//! - **Colors**: ANSI color support for console output
//!
//! # Example
//!
//! ```no_run
//! use logly::prelude::*;
//! use std::path::PathBuf;
//!
//! let config = SinkConfig {
//!     path: Some(PathBuf::from("logs/app.log")),
//!     rotation: Some("daily".to_string()),
//!     size_limit: Some(10 * 1024 * 1024), // 10MB
//!     retention: Some(7), // Keep 7 files
//!     async_write: true,
//!     ..Default::default()
//! };
//! ```

use crate::error::Result;
use crate::filter::Filter;
use crate::format::Formatter;
use crate::level::Level;
use crate::record::LogRecord;
use crate::rotation::{RotationManager, RotationPolicy};
use crossbeam_channel::{Sender, bounded};
use parking_lot::RwLock;
use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use std::sync::Arc;

/// Configuration for a log sink.
///
/// Defines all settings for a single output destination including path,
/// rotation, filtering, formatting, and performance options.
pub struct SinkConfig {
    /// File path (None = console output)
    pub path: Option<PathBuf>,
    /// Rotation interval: "hourly", "daily", "weekly", "monthly", "yearly"
    pub rotation: Option<String>,
    /// Maximum file size before rotation (bytes)
    pub size_limit: Option<u64>,
    /// Number of rotated files to keep (None = unlimited)
    pub retention: Option<usize>,
    /// Minimum log level to accept
    pub filter_min_level: Option<Level>,
    /// Filter by module name
    pub filter_module: Option<String>,
    /// Filter by function name
    pub filter_function: Option<String>,
    /// Enable async writes (recommended for file sinks)
    pub async_write: bool,
    /// Write buffer size in bytes
    pub buffer_size: usize,
    /// Flush interval in milliseconds
    pub flush_interval: u64,
    /// Maximum number of buffered log records
    pub max_buffered_lines: usize,
    /// Custom date/time format pattern
    pub date_style: Option<String>,
    /// Enable timestamp in output
    pub date_enabled: bool,
    /// Custom format template string
    pub format: Option<String>,
    /// Enable JSON output format
    pub json: bool,
    /// Enable ANSI color codes
    pub color: bool,
}

impl Default for SinkConfig {
    fn default() -> Self {
        Self {
            path: None,
            rotation: None,
            size_limit: None,
            retention: None,
            filter_min_level: None,
            filter_module: None,
            filter_function: None,
            async_write: true,
            buffer_size: 8192,
            flush_interval: 100,
            max_buffered_lines: 1000,
            date_style: None,
            date_enabled: false,
            format: None,
            json: false,
            color: true, // Enable colors by default for console
        }
    }
}

/// A log output destination (sink).
///
/// Manages writing log records to console or file with optional filtering,
/// formatting, rotation, and async writes.
pub struct Sink {
    /// Unique sink identifier
    id: usize,
    /// Sink configuration
    config: SinkConfig,
    /// File writer (None for console sinks)
    writer: Arc<RwLock<Option<BufWriter<File>>>>,
    /// Filter for log records
    filter: Filter,
    /// Formatter for log records
    formatter: Formatter,
    /// Whether this sink is enabled
    enabled: Arc<RwLock<bool>>,
    /// Async write channel sender
    sender: Option<Sender<LogRecord>>,
    /// File rotation manager
    rotation_manager: Arc<RwLock<Option<RotationManager>>>,
}

impl Sink {
    /// Sets custom colors for log levels.
    ///
    /// # Arguments
    ///
    /// * `colors` - Map of log levels to ANSI color codes
    pub fn set_level_colors(
        &mut self,
        colors: std::collections::HashMap<crate::level::Level, String>,
    ) {
        self.formatter = self.formatter.clone().with_level_colors(colors);
    }
}

impl Sink {
    /// Creates a new sink with the specified configuration.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique sink identifier
    /// * `config` - Sink configuration
    ///
    /// # Returns
    ///
    /// A new Sink instance, or an error if initialization fails
    pub fn new(id: usize, config: SinkConfig) -> Result<Self> {
        let filter = Filter::new(
            config.filter_min_level,
            config.filter_module.clone(),
            config.filter_function.clone(),
        );

        let formatter = Formatter::new(
            config.format.clone(),
            config.json,
            config.date_enabled,
            config.date_style.clone(),
        )
        .with_color(config.color);

        let writer = if let Some(ref path) = config.path {
            // Create parent directories if they don't exist
            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(parent)?;
            }
            let file = OpenOptions::new().create(true).append(true).open(path)?;
            Some(BufWriter::with_capacity(config.buffer_size, file))
        } else {
            None
        };

        let (sender, writer_arc) = if config.async_write {
            let (s, r) = bounded(config.max_buffered_lines);

            let writer_clone = Arc::new(RwLock::new(writer));
            let writer_ref = Arc::clone(&writer_clone);
            let formatter_clone = formatter.clone();

            std::thread::spawn(move || {
                while let Ok(record) = r.recv() {
                    if let Some(ref mut w) = *writer_ref.write() {
                        let formatted = formatter_clone.format(&record);
                        let _ = writeln!(w, "{}", formatted);
                        let _ = w.flush();
                    }
                }
            });

            (Some(s), writer_clone)
        } else {
            (None, Arc::new(RwLock::new(writer)))
        };

        // Initialize rotation manager
        let rotation_manager = if let Some(ref path) = config.path {
            if config.rotation.is_some() || config.size_limit.is_some() {
                let policy = match (&config.rotation, config.size_limit) {
                    (Some(interval), Some(size)) => RotationPolicy::Both(size, interval.clone()),
                    (Some(interval), None) => RotationPolicy::Time(interval.clone()),
                    (None, Some(size)) => RotationPolicy::Size(size),
                    _ => RotationPolicy::Size(10 * 1024 * 1024), // Default 10MB
                };
                Some(RotationManager::new(path.clone(), policy, config.retention))
            } else {
                None
            }
        } else {
            None
        };

        Ok(Self {
            id,
            config,
            writer: writer_arc,
            filter,
            formatter,
            enabled: Arc::new(RwLock::new(true)),
            sender,
            rotation_manager: Arc::new(RwLock::new(rotation_manager)),
        })
    }

    /// Writes a log record to this sink.
    ///
    /// # Arguments
    ///
    /// * `record` - The log record to write
    /// * `global_console` - Whether console output is globally enabled
    /// * `global_storage` - Whether file storage is globally enabled
    ///
    /// # Returns
    ///
    /// An error if writing fails
    pub fn log(
        &self,
        record: &LogRecord,
        global_console: bool,
        global_storage: bool,
    ) -> Result<()> {
        if !*self.enabled.read() {
            return Ok(());
        }

        if !self.filter.matches(record) {
            return Ok(());
        }

        let formatted = self.formatter.format(record);
        let data_size = formatted.len() as u64;

        // Console output (if no file path and global console enabled)
        if self.config.path.is_none() && global_console {
            println!("{}", formatted);
            return Ok(());
        }

        // File storage (only if global storage enabled)
        if !global_storage {
            return Ok(());
        }

        // Check rotation
        if let Some(ref mut rotation) = *self.rotation_manager.write() {
            if rotation.should_rotate(data_size) {
                rotation.rotate()?;
                // Reopen file after rotation
                if let Some(ref path) = self.config.path {
                    // Create parent directories if they don't exist
                    if let Some(parent) = path.parent() {
                        std::fs::create_dir_all(parent)?;
                    }
                    let file = OpenOptions::new().create(true).append(true).open(path)?;
                    *self.writer.write() =
                        Some(BufWriter::with_capacity(self.config.buffer_size, file));
                }
            }
            rotation.update_size(data_size);
        }

        if let Some(ref sender) = self.sender {
            sender
                .send(record.clone())
                .map_err(|_| crate::error::LoglyError::ChannelSend)?;
        } else if let Some(ref mut writer) = *self.writer.write() {
            writeln!(writer, "{}", formatted)?;
            writer.flush()?;
        }

        Ok(())
    }

    /// Returns the sink's unique identifier.
    pub fn id(&self) -> usize {
        self.id
    }

    /// Enables this sink.
    pub fn enable(&self) {
        *self.enabled.write() = true;
    }

    /// Disables this sink.
    pub fn disable(&self) {
        *self.enabled.write() = false;
    }

    /// Checks if this sink is enabled.
    pub fn is_enabled(&self) -> bool {
        *self.enabled.read()
    }
}
