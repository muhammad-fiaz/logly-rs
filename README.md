<div align="center">
<img src="https://github.com/user-attachments/assets/565fc3dc-dd2c-47a6-bab6-2f545c551f26" alt="logly logo" width="400" />

[![Crates.io](https://img.shields.io/crates/v/logly)](https://crates.io/crates/logly)
[![Documentation](https://docs.rs/logly/badge.svg)](https://docs.rs/logly)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Build Status](https://github.com/muhammad-fiaz/logly-rs/workflows/CI/badge.svg)](https://github.com/muhammad-fiaz/logly-rs/actions)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![GitHub stars](https://img.shields.io/github/stars/muhammad-fiaz/logly-rs)](https://github.com/muhammad-fiaz/logly-rs)
[![GitHub issues](https://img.shields.io/github/issues/muhammad-fiaz/logly-rs)](https://github.com/muhammad-fiaz/logly-rs/issues)
[![GitHub pull requests](https://img.shields.io/github/issues-pr/muhammad-fiaz/logly-rs)](https://github.com/muhammad-fiaz/logly-rs/pulls)
[![GitHub last commit](https://img.shields.io/github/last-commit/muhammad-fiaz/logly-rs)](https://github.com/muhammad-fiaz/logly-rs)

<p><em>High-performance, production-ready structured logging library for Rust</em></p>

**📚 [Documentation](https://muhammad-fiaz.github.io/logly-rs/) | [API Reference](https://docs.rs/logly) | [Quick Start](docs/QUICK_START.md)**

</div>

---

## Overview

logly-rs is a high-performance, production-ready structured logging library for Rust with async support, GPU acceleration, rotation, filtering, callbacks, and comprehensive error handling.

### 🚀 Core Features

- **8 Log Levels**: TRACE (5), DEBUG (10), INFO (20), SUCCESS (25), WARNING (30), ERROR (40), FAIL (45), CRITICAL (50)
- **Custom Log Levels**: Add your own levels with custom priorities and colors
- **Structured Logging**: JSON and custom format support
- **Context Binding**: Persistent and temporary context fields
- **Out-of-Box Ready**: Works immediately with auto-sink enabled by default

### ⚡ Performance & Scalability

- **Async Logging**: Non-blocking writes with configurable buffers
- **GPU/CUDA Support**: Optional GPU acceleration (compile with `--features gpu`)
- **Thread-Safe**: Lock-free operations with concurrent logging support
- **Zero-Copy**: Efficient memory management
- **High Throughput**: 13,000+ operations per second

### 📝 Output Management

- **Multiple Sinks**: Console, file, and custom outputs
- **Auto-Sinks**: Automatic sink initialization (enabled by default)
- **Manual Sink Management**: Full control over sink lifecycle
- **File Rotation**: Time-based (hourly, daily, weekly, monthly, yearly) and size-based rotation
- **Retention Policies**: Automatic cleanup of old log files
- **Global Controls**: Enable/disable console display and file storage globally

### 🔧 Filtering & Formatting

- **Level Filtering**: Filter by minimum log level
- **Module/Function Filtering**: Target specific code sections
- **Custom Formatters**: Template-based formatting with placeholders
- **Colored Output**: ANSI colors with custom color callbacks
- **Custom Time Formats**: YYYY-MM-DD HH:mm:ss.SSS patterns

### 🎯 Advanced Features

- **Callbacks**: Log callbacks, color callbacks, exception callbacks
- **Exception Handling**: Comprehensive error handling with backtraces
- **Auto-Update Check**: Checks for new versions automatically (enabled by default)
- **Debug Mode**: Detailed internal logging for troubleshooting
- **Runtime Configuration**: Change all settings at runtime
- **Enable/Disable**: Toggle logging on/off without removing sinks

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
logly = "0.0.4"

# With GPU support (experimental)
logly = { version = "0.0.4", features = ["gpu"] }
```

## Quick Start

```rust
use logly::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let logger = Logger::new();
    logger.add_sink(SinkConfig::default())?;
    
    logger.info("Application started".to_string())?;
    logger.success("Operation completed!".to_string())?;
    logger.warning("Warning message".to_string())?;
    logger.error("Error occurred".to_string())?;
    
    Ok(())
}
```

## Performance Benchmarks

Real benchmark results from criterion (measured on your system):

| Operation | Average Time | Throughput | Notes |
|-----------|--------------|------------|-------|
| Basic INFO log | 75.6 μs | 13,200 ops/sec | Single message to console |
| File logging | ~150 μs | 6,600 ops/sec | With async write |
| With context | ~130 μs | 7,700 ops/sec | 2-3 bound fields |
| Concurrent (10 threads) | ~500 μs | 2,000 ops/sec | 10 threads × 100 messages |
| Multiple sinks (5) | ~200 μs | 5,000 ops/sec | Console + 4 files |
| TRACE level | ~74 μs | 13,500 ops/sec | Fastest level |
| DEBUG level | ~75 μs | 13,300 ops/sec | |
| INFO level | ~76 μs | 13,200 ops/sec | |
| WARNING level | ~75 μs | 13,300 ops/sec | |
| ERROR level | ~76 μs | 13,200 ops/sec | |

*Benchmarks run with criterion on Windows. Times include formatting, serialization, and I/O.*

### Performance Characteristics

- **Memory**: <1MB base overhead
- **CPU**: Minimal impact with async writes
- **Latency**: Sub-millisecond for most operations
- **Scalability**: Linear scaling up to 10 threads

See [Performance Guide](docs/PERFORMANCE.md) for optimization techniques.

## Platform Support

- ✅ **Windows**: Full support (tested on Windows 11)
- ✅ **Linux**: Full support (Ubuntu 20.04+)
- ✅ **macOS**: Full support (macOS 11+)
- ⚠️ **GPU**: Requires CUDA toolkit (Linux/Windows only)

## Advanced Usage

### File Logging with Rotation

```rust
use logly::prelude::*;
use std::path::PathBuf;

let logger = Logger::new();

let config = SinkConfig {
    path: Some(PathBuf::from("logs/app.log")),
    rotation: Some("daily".to_string()),
    size_limit: Some(10 * 1024 * 1024), // 10MB
    retention: Some(7), // Keep 7 files
    async_write: true,
    ..Default::default()
};

logger.add_sink(config)?;
```

### Custom Log Levels

```rust
// Add custom level with priority between WARNING (30) and ERROR (40)
logger.add_custom_level("NOTICE".to_string(), 35, "96".to_string())?;

// Use custom level
logger.log_custom("NOTICE", "Custom level message".to_string())?;
```

### Callbacks

```rust
// Log callback for monitoring
logger.add_log_callback(|record| {
    if record.level >= Level::Error {
        println!("High severity: {}", record.message);
    }
    Ok(())
});

// Custom color callback
logger.add_color_callback(|level, message| {
    format!("\x1b[1m[{}]\x1b[0m {}", level.as_str(), message)
});

// Exception callback
logger.add_exception_callback(|error, backtrace| {
    eprintln!("Exception: {}\n{}", error, backtrace);
});
```

### Context Binding

```rust
// Bind persistent context
logger.bind("user_id".to_string(), serde_json::json!("12345"));
logger.bind("session".to_string(), serde_json::json!("abc-def"));

logger.info("User action logged".to_string())?;

// Remove binding
logger.unbind("user_id");

// Clear all bindings
logger.clear_bindings();
```

### GPU Acceleration

```rust
let mut config = LoggerConfig::default();
config.enable_gpu = true;
config.gpu_buffer_size = 2 * 1024 * 1024; // 2MB

logger.configure(config);
logger.enable_gpu()?;

println!("{}", logger.gpu_info());
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

See [Configuration Guide](docs/CONFIGURATION.md) for all options.

## Documentation

Full documentation available at: https://muhammad-fiaz.github.io/logly-rs

### Getting Started

- [Installation Guide](docs/INSTALLATION.md) - Complete installation instructions
- [Quick Start](docs/QUICK_START.md) - Get started in minutes
- [Configuration Guide](docs/CONFIGURATION.md) - Complete configuration reference

### Feature Guides

- [Log Levels](docs/guides/LEVELS.md) - All 8 levels and custom levels
- [Sinks](docs/guides/SINKS.md) - Multiple sinks, auto-sinks, manual management
- [Rotation & Retention](docs/guides/ROTATION.md) - File rotation (hourly to yearly)
- [Configuration File](docs/guides/CONFIG_FILE.md) - logly.toml support
- [Template Formatting](docs/guides/FORMATTING.md) - Custom time formats and placeholders
- [Callbacks](docs/guides/CALLBACKS.md) - Log, color, and exception callbacks
- [GPU Support](docs/guides/GPU.md) - GPU/CUDA acceleration
- [Troubleshooting](docs/guides/TROUBLESHOOTING.md) - Common issues and solutions
- [Performance](docs/PERFORMANCE.md) - Optimization techniques

### Project Information

- [Changelog](docs/CHANGELOG.md) - Release history and changes
- [Contributing](CONTRIBUTING.md) - How to contribute
- [License](LICENSE) - MIT License
- [Code of Conduct](CODE_OF_CONDUCT.md) - Community guidelines

## Examples

Run the examples:

```bash
# Basic usage
cargo run --example basic

# Advanced features
cargo run --example advanced

# Complete configuration
cargo run --example configuration

# GPU logging (requires CUDA)
cargo run --example gpu_logging --features gpu

# Callbacks and custom colors
cargo run --example callbacks

# File rotation
cargo run --example rotation

# Custom log levels
cargo run --example custom_levels
```

## Testing

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_level_priority

# Run benchmarks
cargo bench
```

## Error Reporting

If you encounter any bugs or issues:

- **Rust crate**: https://github.com/muhammad-fiaz/logly-rs/issues
- **Python package**: https://github.com/muhammad-fiaz/logly/issues

## Contributing

Contributions welcome! See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Repository

- **Rust**: https://github.com/muhammad-fiaz/logly-rs
- **Python**: https://github.com/muhammad-fiaz/logly

## Version

Current version: **0.0.4**

See [CHANGELOG.md](docs/CHANGELOG.md) for release history.

Check for updates:
```rust
if let Ok(Some(msg)) = logger.check_version() {
    println!("{}", msg);
}
```

## Author

muhammad-fiaz

---

<div align="center">

[![Star History Chart](https://api.star-history.com/svg?repos=muhammad-fiaz/logly-rs&type=Date&bg=transparent)](https://github.com/muhammad-fiaz/logly-rs/)

**⭐ Star the repository if you find logly-rs useful!**

</div>
