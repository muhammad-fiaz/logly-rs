//! File rotation and retention management
//!
//! Provides automatic log file rotation based on size or time intervals,
//! with configurable retention policies to manage disk space.
//!
//! # Rotation Policies
//!
//! - **Size**: Rotate when file reaches specified size
//! - **Time**: Rotate at specified intervals (hourly, daily, weekly, monthly, yearly)
//! - **Both**: Rotate when either size or time threshold is reached
//!
//! # Example
//!
//! ```no_run
//! use logly::rotation::{RotationManager, RotationPolicy};
//! use std::path::PathBuf;
//!
//! let policy = RotationPolicy::Both(10 * 1024 * 1024, "daily".to_string());
//! let mut manager = RotationManager::new(
//!     PathBuf::from("logs/app.log"),
//!     policy,
//!     Some(7) // Keep 7 rotated files
//! );
//! ```

use crate::error::{LoglyError, Result};
use chrono::{DateTime, Utc};
use std::fs;
use std::path::{Path, PathBuf};

/// Policy for determining when to rotate log files.
#[derive(Debug, Clone)]
pub enum RotationPolicy {
    /// Rotate when file size exceeds the specified bytes
    Size(u64),
    /// Rotate at time intervals: "hourly", "daily", "weekly", "monthly", "yearly"
    Time(String),
    /// Rotate when either size or time threshold is reached
    Both(u64, String),
}

/// Manages log file rotation and retention.
///
/// Tracks file size and time, rotates files when thresholds are reached,
/// and applies retention policies to clean up old files.
pub struct RotationManager {
    /// Base path for the log file
    base_path: PathBuf,
    /// Rotation policy (size, time, or both)
    policy: RotationPolicy,
    /// Maximum number of rotated files to keep (None = unlimited)
    retention: Option<usize>,
    /// Current size of the active log file in bytes
    current_size: u64,
    /// Timestamp of the last rotation
    last_rotation: DateTime<Utc>,
}

impl RotationManager {
    /// Creates a new rotation manager.
    ///
    /// # Arguments
    ///
    /// * `base_path` - Path to the log file
    /// * `policy` - Rotation policy (size, time, or both)
    /// * `retention` - Maximum number of rotated files to keep (None = unlimited)
    pub fn new(base_path: PathBuf, policy: RotationPolicy, retention: Option<usize>) -> Self {
        Self {
            base_path,
            policy,
            retention,
            current_size: 0,
            last_rotation: Utc::now(),
        }
    }

    /// Checks if the log file should be rotated.
    ///
    /// # Arguments
    ///
    /// * `additional_size` - Size of data about to be written
    ///
    /// # Returns
    ///
    /// `true` if rotation is needed, `false` otherwise
    pub fn should_rotate(&mut self, additional_size: u64) -> bool {
        match &self.policy {
            RotationPolicy::Size(max_size) => self.current_size + additional_size >= *max_size,
            RotationPolicy::Time(interval) => self.should_rotate_by_time(interval),
            RotationPolicy::Both(max_size, interval) => {
                (self.current_size + additional_size >= *max_size)
                    || self.should_rotate_by_time(interval)
            }
        }
    }

    /// Checks if rotation is needed based on time interval.
    ///
    /// # Arguments
    ///
    /// * `interval` - Time interval string ("hourly", "daily", "weekly", "monthly", "yearly")
    ///
    /// # Returns
    ///
    /// `true` if the interval has elapsed since last rotation
    fn should_rotate_by_time(&self, interval: &str) -> bool {
        let now = Utc::now();
        let duration = now.signed_duration_since(self.last_rotation);

        match interval.to_lowercase().as_str() {
            "hourly" => duration.num_hours() >= 1,
            "daily" => duration.num_days() >= 1,
            "weekly" => duration.num_weeks() >= 1,
            "monthly" => duration.num_days() >= 30,
            "yearly" => duration.num_days() >= 365,
            _ => false,
        }
    }

    /// Rotates the log file by renaming it with a timestamp.
    ///
    /// Creates a new file with the original name and applies retention policy.
    ///
    /// # Returns
    ///
    /// Path to the rotated file, or an error if rotation fails
    pub fn rotate(&mut self) -> Result<PathBuf> {
        let timestamp = Utc::now().format("%Y%m%d_%H%M%S");
        let extension = self
            .base_path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("log");

        let stem = self
            .base_path
            .file_stem()
            .and_then(|s| s.to_str())
            .ok_or_else(|| LoglyError::InvalidConfig("Invalid file path".to_string()))?;

        let parent = self
            .base_path
            .parent()
            .ok_or_else(|| LoglyError::InvalidConfig("Invalid file path".to_string()))?;

        let rotated_path = parent.join(format!("{}_{}.{}", stem, timestamp, extension));

        if self.base_path.exists() {
            fs::rename(&self.base_path, &rotated_path)?;
        }

        self.current_size = 0;
        self.last_rotation = Utc::now();

        if let Some(retention) = self.retention {
            self.apply_retention(parent, stem, extension, retention)?;
        }

        Ok(rotated_path)
    }

    /// Applies retention policy by deleting old rotated files.
    ///
    /// # Arguments
    ///
    /// * `dir` - Directory containing log files
    /// * `stem` - Base filename without extension
    /// * `extension` - File extension
    /// * `max_files` - Maximum number of files to keep
    fn apply_retention(
        &self,
        dir: &Path,
        stem: &str,
        extension: &str,
        max_files: usize,
    ) -> Result<()> {
        let mut log_files: Vec<_> = fs::read_dir(dir)?
            .filter_map(|entry| entry.ok())
            .filter(|entry| {
                if let Some(name) = entry.file_name().to_str() {
                    name.starts_with(stem) && name.ends_with(extension)
                } else {
                    false
                }
            })
            .collect();

        log_files.sort_by_key(|entry| {
            entry
                .metadata()
                .and_then(|m| m.modified())
                .unwrap_or(std::time::SystemTime::UNIX_EPOCH)
        });

        if log_files.len() > max_files {
            for entry in log_files.iter().take(log_files.len() - max_files) {
                fs::remove_file(entry.path())?;
            }
        }

        Ok(())
    }

    /// Updates the current file size by adding the specified bytes.
    ///
    /// # Arguments
    ///
    /// * `size` - Number of bytes to add to current size
    pub fn update_size(&mut self, size: u64) {
        self.current_size += size;
    }

    /// Returns the current file size in bytes.
    pub fn current_size(&self) -> u64 {
        self.current_size
    }
}
