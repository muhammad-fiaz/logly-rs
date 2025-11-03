//! Utility functions
//!
//! Provides helper functions for parsing size limits, colorizing text,
//! and other common operations used throughout the library.

use crate::error::{LoglyError, Result};

/// Parses a human-readable size string into bytes.
/// 
/// Supports units: B, KB/K, MB/M, GB/G, TB/T
/// 
/// # Arguments
/// 
/// * `size_str` - Size string (e.g., "10MB", "5GB", "1024")
/// 
/// # Returns
/// 
/// Size in bytes, or an error if parsing fails
/// 
/// # Examples
/// 
/// ```
/// use logly::utils::parse_size_limit;
/// 
/// assert_eq!(parse_size_limit("100").unwrap(), 100);
/// assert_eq!(parse_size_limit("5KB").unwrap(), 5 * 1024);
/// assert_eq!(parse_size_limit("10MB").unwrap(), 10 * 1024 * 1024);
/// ```
pub fn parse_size_limit(size_str: &str) -> Result<u64> {
    let size_str = size_str.trim().to_uppercase();
    
    let (num_str, unit) = if size_str.ends_with("TB") || size_str.ends_with("GB") || 
                             size_str.ends_with("MB") || size_str.ends_with("KB") {
        let len = size_str.len();
        (&size_str[..len-2], &size_str[len-2..])
    } else if size_str.ends_with('T') || size_str.ends_with('G') || 
              size_str.ends_with('M') || size_str.ends_with('K') || 
              size_str.ends_with('B') {
        let len = size_str.len();
        (&size_str[..len-1], &size_str[len-1..])
    } else {
        (size_str.as_str(), "B")
    };

    let num: u64 = num_str.parse()
        .map_err(|_| LoglyError::InvalidConfig(format!("Invalid size: {}", size_str)))?;

    let multiplier = match unit {
        "B" => 1,
        "K" | "KB" => 1024,
        "M" | "MB" => 1024 * 1024,
        "G" | "GB" => 1024 * 1024 * 1024,
        "T" | "TB" => 1024 * 1024 * 1024 * 1024,
        _ => return Err(LoglyError::InvalidConfig(format!("Invalid unit: {}", unit))),
    };

    Ok(num * multiplier)
}

/// Wraps text with ANSI color codes.
/// 
/// # Arguments
/// 
/// * `text` - Text to colorize
/// * `color_code` - ANSI color code (e.g., "31" for red, "32" for green)
/// 
/// # Returns
/// 
/// Text wrapped with ANSI escape sequences
/// 
/// # Examples
/// 
/// ```
/// use logly::utils::colorize;
/// 
/// let red_text = colorize("Error", "31");
/// let green_text = colorize("Success", "32");
/// ```
pub fn colorize(text: &str, color_code: &str) -> String {
    format!("\x1b[{}m{}\x1b[0m", color_code, text)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_size_limit() {
        assert_eq!(parse_size_limit("100").unwrap(), 100);
        assert_eq!(parse_size_limit("5KB").unwrap(), 5 * 1024);
        assert_eq!(parse_size_limit("10MB").unwrap(), 10 * 1024 * 1024);
        assert_eq!(parse_size_limit("1GB").unwrap(), 1024 * 1024 * 1024);
    }
}
