# Logly Configuration Guide

## Out-of-the-Box Usage

Logly works immediately without any configuration:

```rust
use logly::prelude::*;

let logger = Logger::new();
// Auto-sink is enabled by default - logs to console
logger.info("Works out of the box!".to_string())?;
```

## Global Configuration Options

### LoggerConfig Structure

The `LoggerConfig` struct contains all configuration options:

```rust,ignore
pub struct LoggerConfig {
    // Log level filtering
    pub level: Level,                              // Minimum log level (default: Info)
    
    // Global display controls
    pub global_color_display: bool,                // Enable/disable colors globally (default: true)
    pub global_console_display: bool,              // Enable/disable console output (default: true)
    pub global_file_storage: bool,                 // Enable/disable file storage (default: true)
    
    // Color settings
    pub color: bool,                               // Enable colors (default: true)
    pub level_colors: HashMap<Level, String>,      // Per-level color codes
    
    // Custom levels
    pub custom_levels: HashMap<String, CustomLevel>, // User-defined levels
    
    // Output format
    pub json: bool,                                // JSON output (default: false)
    pub pretty_json: bool,                         // Pretty-print JSON (default: false)
    pub log_compact: bool,                         // Compact format (default: false)
    
    // Display options
    pub console: bool,                             // Console output (default: true)
    pub show_time: bool,                           // Show timestamp (default: true)
    pub show_module: bool,                         // Show module name (default: true)
    pub show_function: bool,                       // Show function name (default: true)
    pub show_filename: bool,                       // Show filename (default: false)
    pub show_lineno: bool,                         // Show line number (default: false)
    
    // Per-level controls
    pub console_levels: HashMap<Level, bool>,      // Per-level console output
    pub time_levels: HashMap<Level, bool>,         // Per-level timestamp display
    pub color_levels: HashMap<Level, bool>,        // Per-level color enable
    pub storage_levels: HashMap<Level, bool>,      // Per-level file storage
    
    // Sink management
    pub auto_sink: bool,                           // Auto-initialize console sink (default: true)
    
    // GPU support (experimental)
    pub enable_gpu: bool,                          // Enable GPU acceleration (default: false)
    pub gpu_buffer_size: usize,                    // GPU buffer size (default: 1MB)
    
    // Features
    pub enable_callbacks: bool,                    // Enable callback system (default: true)
    pub enable_exception_handling: bool,           // Enable exception handling (default: true)
    pub enable_version_check: bool,                // Enable auto-update check (default: true)
    
    // Debug mode
    pub debug_mode: bool,                          // Enable debug logging (default: false)
    pub debug_log_file: Option<PathBuf>,           // Debug log file path (default: None = stderr)
}
```

## Configuration Examples

### 1. Default Configuration (Out of Box)

```rust
let logger = Logger::new();
// Auto-sink enabled, logs to console with colors
logger.info("Ready to use!".to_string())?;
```

### 2. Console Only (No File Storage)

```rust
let mut config = LoggerConfig::default();
config.global_console_display = true;   // Console enabled
config.global_file_storage = false;     // No file storage

logger.configure(config);
logger.info("Console only".to_string())?;
```

### 3. File Storage Only (No Console)

```rust
let mut config = LoggerConfig::default();
config.global_console_display = false;  // No console
config.global_file_storage = true;      // File storage enabled
config.auto_sink = false;               // Disable auto-sink

logger.configure(config);

// Add file sink manually
logger.add_sink(SinkConfig {
    path: Some(PathBuf::from("app.log")),
    ..Default::default()
})?;

logger.info("File only".to_string())?;
```

### 4. No Output (Silent Mode)

```rust
let mut config = LoggerConfig::default();
config.global_console_display = false;  // No console
config.global_file_storage = false;     // No file storage

logger.configure(config);
logger.info("Not displayed or stored".to_string())?;
```

### 5. Debug Mode with File Logging

```rust
let mut config = LoggerConfig::default();
config.debug_mode = true;
config.debug_log_file = Some(PathBuf::from("logly_debug.log"));

logger.configure(config);
// All logly internal operations logged to logly_debug.log
```

### 6. Custom Colors

```rust
let mut config = LoggerConfig::default();
config.level_colors.insert(Level::Info, "92".to_string());    // Bright green
config.level_colors.insert(Level::Error, "91".to_string());   // Bright red

logger.configure(config);
```

### 7. Disable Colors Globally

```rust
let mut config = LoggerConfig::default();
config.global_color_display = false;

logger.configure(config);
```

### 8. JSON Output

```rust
let mut config = LoggerConfig::default();
config.json = true;
config.pretty_json = true;

logger.configure(config);
```

## File Rotation Configuration

### Rotation Intervals

- `hourly` - Rotate every hour
- `daily` - Rotate every day
- `weekly` - Rotate every week
- `monthly` - Rotate every 30 days
- `yearly` - Rotate every 365 days

### Size-Based Rotation

```rust
SinkConfig {
    path: Some(PathBuf::from("app.log")),
    size_limit: Some(10 * 1024 * 1024),  // 10MB
    retention: Some(5),                   // Keep 5 files
    ..Default::default()
}
```

### Time-Based Rotation

```rust
SinkConfig {
    path: Some(PathBuf::from("app.log")),
    rotation: Some("daily".to_string()),
    retention: Some(7),  // Keep 7 days
    ..Default::default()
}
```

### Combined Rotation

```rust
SinkConfig {
    path: Some(PathBuf::from("app.log")),
    rotation: Some("daily".to_string()),
    size_limit: Some(10 * 1024 * 1024),  // Rotate if daily OR 10MB
    retention: Some(7),
    ..Default::default()
}
```

### Yearly Rotation Example

```rust
SinkConfig {
    path: Some(PathBuf::from("yearly.log")),
    rotation: Some("yearly".to_string()),
    retention: Some(5),  // Keep 5 years
    ..Default::default()
}
```

## Runtime Configuration Changes

All configuration can be changed at runtime:

```rust
// Initial configuration
let mut config = LoggerConfig::default();
config.level = Level::Info;
logger.configure(config);

logger.info("Info message".to_string())?;

// Change at runtime
let mut new_config = LoggerConfig::default();
new_config.level = Level::Warning;
logger.configure(new_config);

logger.info("Not shown (below WARNING)".to_string())?;
logger.warning("Shown".to_string())?;
```

## Enable/Disable at Runtime

```rust
logger.enable();
logger.info("Logged".to_string())?;

logger.disable();
logger.info("Not logged".to_string())?;

logger.enable();
logger.info("Logged again".to_string())?;
```

## Auto-Sink Behavior

By default, `auto_sink` is enabled:

```rust
let logger = Logger::new();
// Automatically has a console sink
logger.info("Works immediately!".to_string())?;
```

Disable auto-sink for manual control:

```rust
let mut config = LoggerConfig::default();
config.auto_sink = false;
logger.configure(config);

// Now you must add sinks manually
logger.add_sink(SinkConfig::default())?;
```

## Global Settings Interaction

### Priority Rules

1. If `global_console_display = false` AND `global_file_storage = false`:
   - No output anywhere (silent mode)

2. If `global_console_display = true` AND `global_file_storage = false`:
   - Console output only, no file storage

3. If `global_console_display = false` AND `global_file_storage = true`:
   - File storage only, no console output

4. If both are `true`:
   - Both console and file storage work normally

### Example: Console Display Control

```rust
// Enable console
config.global_console_display = true;
logger.configure(config);
logger.info("Shows on console".to_string())?;

// Disable console
config.global_console_display = false;
logger.configure(config);
logger.info("Not shown on console".to_string())?;
```

### Example: File Storage Control

```rust
// Enable file storage
config.global_file_storage = true;
logger.configure(config);
logger.add_sink(SinkConfig {
    path: Some(PathBuf::from("app.log")),
    ..Default::default()
})?;
logger.info("Stored in file".to_string())?;

// Disable file storage
config.global_file_storage = false;
logger.configure(config);
logger.info("Not stored in file".to_string())?;
```

## Version Checking

Enabled by default:

```rust
// Check for updates
if let Ok(Some(msg)) = logger.check_version() {
    println!("{}", msg);
}

// Disable version checking
let mut config = LoggerConfig::default();
config.enable_version_check = false;
logger.configure(config);
```

## GPU Support (Experimental)

GPU support is optional and requires compilation with `--features gpu`:

```rust
let mut config = LoggerConfig::default();
config.enable_gpu = true;
config.gpu_buffer_size = 2 * 1024 * 1024;  // 2MB

logger.configure(config);
logger.enable_gpu()?;
```

## Complete Example

```rust
use logly::prelude::*;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let logger = Logger::new();
    
    let mut config = LoggerConfig::default();
    
    // Global controls
    config.global_color_display = true;
    config.global_console_display = true;
    config.global_file_storage = true;
    
    // Log level
    config.level = Level::Debug;
    
    // Display options
    config.show_time = true;
    config.show_module = true;
    
    // Features
    config.enable_callbacks = true;
    config.enable_exception_handling = true;
    config.enable_version_check = true;
    config.debug_mode = false;
    
    // Auto-sink
    config.auto_sink = true;
    
    logger.configure(config);
    
    // Add file sink with rotation
    logger.add_sink(SinkConfig {
        path: Some(PathBuf::from("logs/app.log")),
        rotation: Some("daily".to_string()),
        size_limit: Some(10 * 1024 * 1024),
        retention: Some(7),
        ..Default::default()
    })?;
    
    // Use logger
    logger.debug("Debug message".to_string())?;
    logger.info("Info message".to_string())?;
    logger.warning("Warning message".to_string())?;
    logger.error("Error message".to_string())?;
    
    Ok(())
}
```

## Summary

- ✅ Works out of box with auto-sink
- ✅ Global console display control
- ✅ Global file storage control
- ✅ All rotation intervals (hourly to yearly)
- ✅ Runtime configuration changes
- ✅ Enable/disable at runtime
- ✅ Debug mode with file logging
- ✅ Auto-update checking (enabled by default)
- ✅ Full customization of all features
- ✅ Professional production-ready configuration
