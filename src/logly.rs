use std::fs;
use std::io::Write;
use std::path::{ PathBuf};
use std::sync::Mutex;
use std::fmt;

// Define log levels
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum LogLevel {
    Info,
    Warn,
    Error,
    Debug,
    Critical,
    Fatal,
    Trace,
}

// Implement the Display trait for LogLevel
impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LogLevel::Info => write!(f, "Info"),
            LogLevel::Warn => write!(f, "Warn"),
            LogLevel::Error => write!(f, "Error"),
            LogLevel::Debug => write!(f, "Debug"),
            LogLevel::Critical => write!(f, "Critical"),
            LogLevel::Fatal => write!(f, "Fatal"),
            LogLevel::Trace => write!(f, "Trace"),
        }
    }
}

// Define log colors
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum LogColor {
    Red,
    Yellow,
    Cyan,
    Blue,
    White,
    Critical,
}

// Struct to represent the logger
pub struct Logger {
    file: Mutex<Option<fs::File>>,
    color_enabled: bool,
    default_file_path: Option<PathBuf>,
    default_max_file_size: u64,
}

impl Logger {
    // Create a new Logger instance
    pub fn new() -> Self {
        Logger {
            file: Mutex::new(None),
            color_enabled: true,
            default_file_path: None,
            default_max_file_size: 100,
        }
    }

    // Start logging (open the log file)
    pub fn start_logging(&self, file_path: &str) -> std::io::Result<()> {
        let file = fs::File::create(file_path)?;
        *self.file.lock().unwrap() = Some(file);
        Ok(())
    }

    // Stop logging (close the log file)
    pub fn stop_logging(&self) {
        *self.file.lock().unwrap() = None;
    }

    // Set default file path and max file size
    pub fn set_default_file_path(&mut self, path: &str) {
        self.default_file_path = Some(PathBuf::from(path));
    }

    pub fn set_default_max_file_size(&mut self, max_size: u64) {
        self.default_max_file_size = max_size;
    }

    // Log a message with a specified level and color
    fn log_message(&self, level: LogLevel, key: &str, value: &str, color: LogColor) {
        let color_code = if self.color_enabled {
            match color {
                LogColor::Red => "\x1b[31m",
                LogColor::Yellow => "\x1b[33m",
                LogColor::Cyan => "\x1b[36m",
                LogColor::Blue => "\x1b[34m",
                LogColor::White => "\x1b[37m",
                LogColor::Critical => "\x1b[1;31m",
            }
        } else {
            ""
        };

        let reset_color = if self.color_enabled { "\x1b[0m" } else { "" };

        let log_message = format!(
            "{}[{}]: {} - {}{}{}\n",
            color_code, level, key, value, reset_color, reset_color
        );

        print!("{}", log_message);

        // Write to the log file if it's open
        if let Some(ref mut file) = *self.file.lock().unwrap() {
            if let Err(err) = file.write_all(log_message.as_bytes()) {
                eprintln!("Error writing to log file: {}", err);
            }
        }
    }

    // Log methods for various levels and colors
    pub fn info(&self, key: &str, value: &str, color: LogColor) {
        self.log_message(LogLevel::Info, key, value, color);
    }

    pub fn warn(&self, key: &str, value: &str, color: LogColor) {
        self.log_message(LogLevel::Warn, key, value, color);
    }

    pub fn error(&self, key: &str, value: &str, color: LogColor) {
        self.log_message(LogLevel::Error, key, value, color);
    }

    pub fn debug(&self, key: &str, value: &str, color: LogColor) {
        self.log_message(LogLevel::Debug, key, value, color);
    }

    pub fn critical(&self, key: &str, value: &str, color: LogColor) {
        self.log_message(LogLevel::Critical, key, value, color);
    }

    pub fn fatal(&self, key: &str, value: &str, color: LogColor) {
        self.log_message(LogLevel::Fatal, key, value, color);
    }

    pub fn trace(&self, key: &str, value: &str, color: LogColor) {
        self.log_message(LogLevel::Trace, key, value, color);
    }

    pub fn log(&self, key: &str, value: &str, color: LogColor) {
        self.log_message(LogLevel::Info, key, value, color);
    }

    // Set color enabled or disabled
    pub fn set_color_enabled(&mut self, color_enabled: bool) {
        self.color_enabled = color_enabled;
    }
}
