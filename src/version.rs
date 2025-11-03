//! Auto-update version checking
//!
//! Checks crates.io for newer versions of logly and notifies users.
//! Can be enabled/disabled via configuration.
//!
//! # Example
//!
//! ```no_run
//! use logly::VersionChecker;
//!
//! let mut checker = VersionChecker::new(true);
//! if let Ok(Some(msg)) = checker.check_for_updates() {
//!     println!("{}", msg);
//! }
//! ```

use crate::error::{LoglyError, Result};

/// Current crate version from Cargo.toml
const CRATE_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Version checker for auto-update notifications.
///
/// Queries crates.io API to check for newer versions and notifies users.
pub struct VersionChecker {
    /// Whether version checking is enabled
    enabled: bool,
}

impl VersionChecker {
    /// Creates a new version checker.
    ///
    /// # Arguments
    ///
    /// * `enabled` - Whether version checking is enabled
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }

    /// Checks for newer versions on crates.io.
    ///
    /// # Returns
    ///
    /// A message if a newer version is available, None if up-to-date or disabled,
    /// or an error if the check fails
    pub fn check_for_updates(&self) -> Result<Option<String>> {
        if !self.enabled {
            return Ok(None);
        }

        #[cfg(feature = "auto-update-check")]
        {
            match self.fetch_latest_version() {
                Ok(latest) => {
                    if self.is_newer(&latest, CRATE_VERSION) {
                        Ok(Some(format!(
                            "\n\u{2139}\u{fe0f}  A new version of {} is available: {} (current: {})\n   Update with: cargo update -p {}\n",
                            env!("CARGO_PKG_NAME"), latest, CRATE_VERSION, env!("CARGO_PKG_NAME")
                        )))
                    } else {
                        Ok(None)
                    }
                }
                Err(_) => Ok(None),
            }
        }

        #[cfg(not(feature = "auto-update-check"))]
        Ok(None)
    }

    /// Fetches the latest version from crates.io API.
    ///
    /// # Returns
    ///
    /// The latest version string, or an error if the request fails
    #[cfg(feature = "auto-update-check")]
    fn fetch_latest_version(&self) -> Result<String> {
        let url = format!("https://crates.io/api/v1/crates/{}", env!("CARGO_PKG_NAME"));

        match ureq::get(&url).call() {
            Ok(response) => {
                let body = response.into_string().map_err(|e| {
                    LoglyError::VersionCheckError(format!("Failed to read response: {}", e))
                })?;

                let json: serde_json::Value = serde_json::from_str(&body).map_err(|e| {
                    LoglyError::VersionCheckError(format!("Failed to parse JSON: {}", e))
                })?;

                json["crate"]["max_version"]
                    .as_str()
                    .map(|s| s.to_string())
                    .ok_or_else(|| {
                        LoglyError::VersionCheckError("Version not found in response".to_string())
                    })
            }
            Err(e) => Err(LoglyError::VersionCheckError(format!(
                "HTTP request failed: {}",
                e
            ))),
        }
    }

    /// Compares two version strings to determine if the first is newer.
    ///
    /// # Arguments
    ///
    /// * `latest` - Latest version string
    /// * `current` - Current version string
    ///
    /// # Returns
    ///
    /// `true` if latest is newer than current
    #[allow(dead_code)]
    fn is_newer(&self, latest: &str, current: &str) -> bool {
        let parse_version =
            |v: &str| -> Vec<u32> { v.split('.').filter_map(|s| s.parse().ok()).collect() };

        let latest_parts = parse_version(latest);
        let current_parts = parse_version(current);

        for (l, c) in latest_parts.iter().zip(current_parts.iter()) {
            if l > c {
                return true;
            } else if l < c {
                return false;
            }
        }

        latest_parts.len() > current_parts.len()
    }

    /// Returns the current crate version.
    pub fn current_version() -> &'static str {
        CRATE_VERSION
    }

    /// Enables version checking.
    pub fn enable(&mut self) {
        self.enabled = true;
    }

    /// Disables version checking.
    pub fn disable(&mut self) {
        self.enabled = false;
    }
}

impl Default for VersionChecker {
    fn default() -> Self {
        Self::new(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_comparison() {
        let checker = VersionChecker::new(false);
        assert!(checker.is_newer("0.2.0", "0.1.9"));
        assert!(checker.is_newer("1.0.0", "0.9.9"));
        assert!(!checker.is_newer("0.1.5", "0.1.9"));
        assert!(!checker.is_newer("0.1.7", "0.1.7"));
    }
}
