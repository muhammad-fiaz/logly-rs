//! Log record formatting with color support
//!
//! This module handles the formatting of log records into human-readable strings.
//! Supports custom format templates, JSON output, time formatting, and ANSI colors.

use crate::level::Level;
use crate::record::LogRecord;
use serde_json;
use std::collections::HashMap;

/// Formatter for converting log records to formatted strings.
///
/// Supports multiple output formats including plain text, JSON, and custom templates.
/// Handles ANSI color codes for console output and custom time formatting.
#[derive(Clone)]
pub struct Formatter {
    /// Optional custom format template string
    format_string: Option<String>,
    /// Enable JSON output format
    json: bool,
    /// Enable timestamp in output
    date_enabled: bool,
    /// Custom date/time format pattern
    date_style: Option<String>,
    /// Enable ANSI color codes
    color_enabled: bool,
    /// Custom colors for each log level
    level_colors: HashMap<Level, String>,
}

impl Formatter {
    pub fn new(
        format_string: Option<String>,
        json: bool,
        date_enabled: bool,
        date_style: Option<String>,
    ) -> Self {
        let mut level_colors = HashMap::new();
        for level in Level::all_levels() {
            level_colors.insert(level, level.default_color().to_string());
        }

        Self {
            format_string,
            json,
            date_enabled,
            date_style,
            color_enabled: true,
            level_colors,
        }
    }

    pub fn with_color(mut self, enabled: bool) -> Self {
        self.color_enabled = enabled;
        self
    }

    pub fn with_level_colors(mut self, colors: HashMap<Level, String>) -> Self {
        self.level_colors = colors;
        self
    }

    pub fn format(&self, record: &LogRecord) -> String {
        if self.json {
            return serde_json::to_string(record).unwrap_or_else(|_| "{}".to_string());
        }

        if let Some(ref fmt) = self.format_string {
            return self.apply_format(fmt, record);
        }

        let mut output = String::new();

        if self.date_enabled {
            let time_format = self.date_style.as_deref().unwrap_or("%Y-%m-%d %H:%M:%S");
            output.push_str(&format!(
                "{} | ",
                self.format_time(&record.timestamp, time_format)
            ));
        }

        // Apply color to level if enabled
        let level_str = if self.color_enabled {
            let color = self
                .level_colors
                .get(&record.level)
                .map(|s| s.as_str())
                .unwrap_or(record.level.default_color());
            self.colorize_level(record.level.as_str(), color)
        } else {
            record.level.as_str().to_string()
        };

        output.push_str(&format!("[{}] ", level_str));
        output.push_str(&record.message);

        for (key, value) in &record.fields {
            output.push_str(&format!(" | {}={}", key, value));
        }

        output
    }

    fn colorize_level(&self, text: &str, color_code: &str) -> String {
        format!("\x1b[{}m{}\x1b[0m", color_code, text)
    }

    fn format_time(&self, timestamp: &chrono::DateTime<chrono::Utc>, pattern: &str) -> String {
        // Support custom time format patterns
        let mut result = pattern.to_string();

        // Year patterns
        result = result.replace("YYYY", &timestamp.format("%Y").to_string());
        result = result.replace("YY", &timestamp.format("%y").to_string());

        // Month patterns
        result = result.replace("MMMM", &timestamp.format("%B").to_string());
        result = result.replace("MMM", &timestamp.format("%b").to_string());
        result = result.replace("MM", &timestamp.format("%m").to_string());

        // Day patterns
        result = result.replace("dddd", &timestamp.format("%A").to_string());
        result = result.replace("ddd", &timestamp.format("%a").to_string());
        result = result.replace("DD", &timestamp.format("%d").to_string());

        // Hour patterns
        result = result.replace("HH", &timestamp.format("%H").to_string());
        result = result.replace("hh", &timestamp.format("%I").to_string());

        // Minute/Second patterns
        result = result.replace("mm", &timestamp.format("%M").to_string());
        result = result.replace("ss", &timestamp.format("%S").to_string());

        // Milliseconds/Microseconds
        result = result.replace("SSS", &timestamp.format("%3f").to_string());
        result = result.replace("SSSSSS", &timestamp.format("%6f").to_string());

        // AM/PM
        result = result.replace("A", &timestamp.format("%p").to_string());
        result = result.replace("a", &timestamp.format("%P").to_string());

        // Timezone
        result = result.replace("ZZ", &timestamp.format("%:z").to_string());
        result = result.replace("Z", &timestamp.format("%z").to_string());

        result
    }

    fn apply_format(&self, fmt: &str, record: &LogRecord) -> String {
        let mut result = fmt.to_string();

        // Handle time with custom format: {time:YYYY-MM-DD HH:mm:ss}
        if result.contains("{time:")
            && let Some(start) = result.find("{time:")
            && let Some(end) = result[start..].find('}')
        {
            let time_pattern = &result[start + 6..start + end];
            let formatted_time = self.format_time(&record.timestamp, time_pattern);
            result = result.replace(&format!("{{time:{}}}", time_pattern), &formatted_time);
        }

        // Default time format
        result = result.replace("{time}", &record.timestamp.to_rfc3339());

        // Apply color to level in custom format
        let level_str = if self.color_enabled {
            let color = self
                .level_colors
                .get(&record.level)
                .map(|s| s.as_str())
                .unwrap_or(record.level.default_color());
            self.colorize_level(record.level.as_str(), color)
        } else {
            record.level.as_str().to_string()
        };
        result = result.replace("{level}", &level_str);
        result = result.replace("{message}", &record.message);

        if let Some(ref module) = record.module {
            result = result.replace("{module}", module);
        }

        if let Some(ref function) = record.function {
            result = result.replace("{function}", function);
        }

        if let Some(ref filename) = record.filename {
            result = result.replace("{filename}", filename);
        }

        if let Some(lineno) = record.lineno {
            result = result.replace("{lineno}", &lineno.to_string());
        }

        // Add extra fields
        for (key, value) in &record.fields {
            result = result.replace(&format!("{{{}}}", key), &value.to_string());
        }

        result
    }
}
