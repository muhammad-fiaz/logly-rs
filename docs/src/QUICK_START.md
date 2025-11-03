# Quick Start Guide

Get started with logly-rs in minutes.

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
logly = "0.0.4"
```

## Basic Usage

### 1. Simple Console Logging

```rust
use logly::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let logger = Logger::new();
    logger.add_sink(SinkConfig::default())?;
    
    logger.info("Hello, logly!".to_string())?;
    logger.success("Operation completed!".to_string())?;
    logger.warning("Be careful!".to_string())?;
    logger.error("Something went wrong!".to_string())?;
    
    Ok(())
}
```

### 2. File Logging

```rust
use logly::prelude::*;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let logger = Logger::new();
    
    let config = SinkConfig {
        path: Some(PathBuf::from("app.log")),
        ..Default::default()
    };
    logger.add_sink(config)?;
    
    logger.info("Logging to file!".to_string())?;
    
    Ok(())
}
```

### 3. All Log Levels

```rust
logger.trace("Detailed trace".to_string())?;      // Priority 5
logger.debug("Debug info".to_string())?;          // Priority 10
logger.info("Information".to_string())?;          // Priority 20
logger.success("Success!".to_string())?;          // Priority 25
logger.warning("Warning".to_string())?;           // Priority 30
logger.error("Error occurred".to_string())?;      // Priority 40
logger.fail("Operation failed".to_string())?;     // Priority 45
logger.critical("Critical!".to_string())?;        // Priority 50
```

## Common Patterns

### File Rotation

```rust
let config = SinkConfig {
    path: Some(PathBuf::from("logs/app.log")),
    rotation: Some("daily".to_string()),
    retention: Some(7), // Keep 7 days
    ..Default::default()
};
logger.add_sink(config)?;
```

### JSON Logging

```rust
let mut config = LoggerConfig::default();
config.json = true;
logger.configure(config);

logger.add_sink(SinkConfig::default())?;
logger.info("JSON formatted".to_string())?;
```

### Context Binding

```rust
logger.bind("user_id".to_string(), serde_json::json!("12345"));
logger.bind("request_id".to_string(), serde_json::json!("req-abc"));

logger.info("User action".to_string())?;
// Logs include user_id and request_id automatically
```

### Custom Log Levels

```rust
logger.add_custom_level("NOTICE".to_string(), 35, "96".to_string())?;
logger.log_custom("NOTICE", "Custom level message".to_string())?;
```

### Callbacks

```rust
// Log callback
logger.add_log_callback(|record| {
    println!("Logged: {}", record.message);
    Ok(())
});

// Color callback
logger.add_color_callback(|level, message| {
    format!("[{}] {}", level.as_str(), message)
});

// Exception callback
logger.add_exception_callback(|error, backtrace| {
    eprintln!("Exception: {}", error);
});
```

### Multiple Sinks

```rust
// Console sink
logger.add_sink(SinkConfig::default())?;

// File sink
logger.add_sink(SinkConfig {
    path: Some(PathBuf::from("app.log")),
    ..Default::default()
})?;

// Error-only file
logger.add_sink(SinkConfig {
    path: Some(PathBuf::from("errors.log")),
    level: Some(Level::Error),
    ..Default::default()
})?;
```

## Configuration

### Basic Configuration

```rust
let mut config = LoggerConfig::default();
config.level = Level::Debug;
config.color = true;
config.json = false;
logger.configure(config);
```

### Global Controls

```rust
let mut config = LoggerConfig::default();
config.global_console_display = true;  // Enable console
config.global_file_storage = true;     // Enable file storage
config.global_color_display = true;    // Enable colors
logger.configure(config);
```

### Configuration File

Create `logly.toml`:

```toml
[logly.configuration]
level = "DEBUG"
auto_sink = true

[logly.display]
color = true
global_console_display = true
global_file_storage = true

[logly.features]
enable_callbacks = true
enable_exception_handling = true
enable_version_check = true
```

Load automatically:

```rust
let logger = Logger::new(); // Loads logly.toml automatically
```

Or specify path:

```rust
let logger = Logger::with_config_file(PathBuf::from("custom.toml"))?;
```

## Advanced Features

### Async Logging

```rust
let config = SinkConfig {
    path: Some(PathBuf::from("async.log")),
    async_write: true,
    ..Default::default()
};
logger.add_sink(config)?;
```

### GPU Acceleration

```toml
[dependencies]
logly = { version = "0.0.4", features = ["gpu"] }
```

```rust
logger.enable_gpu()?;
println!("{}", logger.gpu_info());
```

### Debug Mode

```rust
logger.enable_debug();
// All internal operations logged to stderr or file
```

### Level Filtering

```rust
let mut config = LoggerConfig::default();
config.level = Level::Warning; // Only WARNING and above
logger.configure(config);
```

## Complete Example

```rust
use logly::prelude::*;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create logger
    let logger = Logger::new();
    
    // Configure
    let mut config = LoggerConfig::default();
    config.level = Level::Debug;
    config.color = true;
    config.enable_callbacks = true;
    logger.configure(config);
    
    // Add sinks
    logger.add_sink(SinkConfig::default())?; // Console
    logger.add_sink(SinkConfig {
        path: Some(PathBuf::from("logs/app.log")),
        rotation: Some("daily".to_string()),
        retention: Some(7),
        ..Default::default()
    })?;
    
    // Bind context
    logger.bind("app".to_string(), serde_json::json!("myapp"));
    logger.bind("version".to_string(), serde_json::json!("1.0.0"));
    
    // Add callback
    logger.add_log_callback(|record| {
        if record.level >= Level::Error {
            // Send alert
        }
        Ok(())
    });
    
    // Log messages
    logger.info("Application started".to_string())?;
    logger.success("Initialization complete".to_string())?;
    
    // Simulate work
    for i in 0..10 {
        logger.debug(format!("Processing item {}", i))?;
    }
    
    logger.success("All items processed".to_string())?;
    
    Ok(())
}
```

## Next Steps

- Read [Installation Guide](../INSTALLATION.md)
- Explore [Configuration Guide](../CONFIGURATION.md)
- Check [API Documentation](https://docs.rs/logly)
- Review [Examples](../examples/)
- Read feature guides:
  - [Log Levels](guides/LEVELS.md)
  - [Sinks](guides/SINKS.md)
  - [Rotation](guides/ROTATION.md)
  - [Callbacks](guides/CALLBACKS.md)
  - [GPU Support](guides/GPU.md)

## Troubleshooting

See [Troubleshooting Guide](guides/TROUBLESHOOTING.md) for common issues.

## Getting Help

- **Documentation**: https://muhammad-fiaz.github.io/logly-rs
- **Issues**: https://github.com/muhammad-fiaz/logly-rs/issues
- **Repository**: https://github.com/muhammad-fiaz/logly-rs
