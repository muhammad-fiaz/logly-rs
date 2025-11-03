# Sinks Guide

## Overview

Sinks are output destinations for log messages. Logly supports multiple sinks simultaneously.

## Auto-Sink

Enabled by default - automatically creates a console sink:

```rust
let logger = Logger::new();
// Auto-sink active - logs to console immediately
logger.info("Works out of box!".to_string())?;
```

Disable auto-sink:
```rust
let mut config = LoggerConfig::default();
config.auto_sink = false;
logger.configure(config);
```

## Manual Sink Management

### Add Sink
```rust
let id = logger.add_sink(SinkConfig::default())?;
```

### Remove Sink
```rust
logger.remove_sink(id);
```

### Remove All Sinks
```rust
let count = logger.remove_all_sinks();
```

### List Sinks
```rust
let ids = logger.list_sinks();
let count = logger.get_sink_count();
```

## Multiple Sinks

Add multiple sinks for different outputs:

```rust
// Console sink
let console_id = logger.add_sink(SinkConfig::default())?;

// File sink
let file_id = logger.add_sink(SinkConfig {
    path: Some(PathBuf::from("logs/app.log")),
    ..Default::default()
})?;

// Error-only file
let error_id = logger.add_sink(SinkConfig {
    path: Some(PathBuf::from("logs/errors.log")),
    filter_min_level: Some(Level::Error),
    ..Default::default()
})?;
```

## Sink Configuration

```rust
pub struct SinkConfig {
    pub path: Option<PathBuf>,              // File path (None = console)
    pub rotation: Option<String>,           // Rotation interval
    pub size_limit: Option<u64>,            // Size threshold
    pub retention: Option<usize>,           // Files to keep
    pub filter_min_level: Option<Level>,    // Minimum level
    pub filter_module: Option<String>,      // Module filter
    pub filter_function: Option<String>,    // Function filter
    pub async_write: bool,                  // Async mode
    pub buffer_size: usize,                 // Buffer size
    pub format: Option<String>,             // Custom format
    pub json: bool,                         // JSON output
}
```

## File Formats

Logly automatically creates directories and supports multiple formats:

### .log Files
```rust
SinkConfig {
    path: Some(PathBuf::from("logs/app.log")),
    ..Default::default()
}
```

### .txt Files
```rust
SinkConfig {
    path: Some(PathBuf::from("logs/app.txt")),
    ..Default::default()
}
```

### .json Files
```rust
SinkConfig {
    path: Some(PathBuf::from("logs/app.json")),
    json: true,
    ..Default::default()
}
```

### Custom Extensions
```rust
SinkConfig {
    path: Some(PathBuf::from("logs/app.custom")),
    ..Default::default()
}
```

## Automatic Directory Creation

Logly automatically creates parent directories:

```rust
// Creates logs/app/production/ if it doesn't exist
SinkConfig {
    path: Some(PathBuf::from("logs/app/production/app.log")),
    ..Default::default()
}
```

## Filtering

### Level Filtering
```rust
SinkConfig {
    filter_min_level: Some(Level::Warning),
    ..Default::default()
}
```

### Module Filtering
```rust
SinkConfig {
    filter_module: Some("my_module".to_string()),
    ..Default::default()
}
```

### Function Filtering
```rust
SinkConfig {
    filter_function: Some("my_function".to_string()),
    ..Default::default()
}
```

## Async vs Sync

### Async (Default)
```rust
SinkConfig {
    async_write: true,
    buffer_size: 8192,
    ..Default::default()
}
```

### Sync
```rust
SinkConfig {
    async_write: false,
    ..Default::default()
}
```

## Examples

### Console Only
```rust
logger.add_sink(SinkConfig::default())?;
```

### File with Rotation
```rust
logger.add_sink(SinkConfig {
    path: Some(PathBuf::from("logs/app.log")),
    rotation: Some("daily".to_string()),
    retention: Some(7),
    ..Default::default()
})?;
```

### JSON Logging
```rust
logger.add_sink(SinkConfig {
    path: Some(PathBuf::from("logs/app.json")),
    json: true,
    ..Default::default()
})?;
```

### Error-Only Sink
```rust
logger.add_sink(SinkConfig {
    path: Some(PathBuf::from("logs/errors.log")),
    filter_min_level: Some(Level::Error),
    ..Default::default()
})?;
```
