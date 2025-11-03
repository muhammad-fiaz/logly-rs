# Configuration File Guide (logly.toml)

## Overview

Logly supports configuration files for persistent settings. Configuration priority:

1. **Manual configuration** (highest priority) - `logger.configure(config)`
2. **Config file** (medium priority) - `logly.toml`
3. **Default configuration** (lowest priority) - Built-in defaults

## Automatic Scanning

By default, Logly scans for `logly.toml` in the project root:

```rust
let logger = Logger::new();
// Automatically loads logly.toml if it exists
```

## Disable Automatic Scanning

```rust
let logger = Logger::new();
logger.disable_config_file_scan();
```

## Custom Config File Path

```rust
let logger = Logger::with_config_file(PathBuf::from("config/custom.toml"))?;
```

## Configuration File Format

Create `logly.toml` in your project root:

```toml
[logly.configuration]
# Log level (TRACE, DEBUG, INFO, SUCCESS, WARNING, ERROR, FAIL, CRITICAL)
level = "INFO"

[logly.display]
# Global display controls
global_color_display = true
global_console_display = true
global_file_storage = true

# Color settings
color = true

# Display options
console = true
show_time = true
show_module = true
show_function = false
show_filename = false
show_lineno = false

[logly.format]
# Output format
json = false
pretty_json = false
log_compact = false

[logly.sinks]
# Sink management
auto_sink = true

[logly.gpu]
# GPU support (experimental)
enable_gpu = false
gpu_buffer_size = 1048576  # 1MB

[logly.features]
# Features
enable_callbacks = true
enable_exception_handling = true
enable_version_check = true

[logly.debug]
# Debug mode
debug_mode = false
debug_log_file = "logs/logly_debug.log"
```

## Configuration Priority Example

```rust
// 1. Default: Level::Info
let logger = Logger::new();

// 2. Config file overrides default: Level::Debug
// logly.toml: level = "DEBUG"

// 3. Manual config overrides file: Level::Warning
let mut config = LoggerConfig::default();
config.level = Level::Warning;
logger.configure(config);
```

## Duplicate Config Files

If multiple config files exist (`logly.toml`, `Logly.toml`, `LOGLY.toml`):
- Warning is displayed
- First found file is used
- Recommended: Use only one config file

## Complete Example

### logly.toml
```toml
level = "DEBUG"
global_console_display = true
global_file_storage = true
color = true
show_time = true
show_module = true
auto_sink = true
enable_version_check = true
debug_mode = false
```

### main.rs
```rust
use logly::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Loads logly.toml automatically
    let logger = Logger::new();
    
    // Config file settings are applied
    logger.debug("Debug enabled from config file".to_string())?;
    
    // Override config file settings
    let mut config = LoggerConfig::default();
    config.level = Level::Warning;
    logger.configure(config);
    
    logger.debug("Not shown (manual override)".to_string())?;
    logger.warning("Shown".to_string())?;
    
    Ok(())
}
```

## Use Cases

### Development Config
```toml
level = "TRACE"
debug_mode = true
debug_log_file = "logs/debug.log"
```

### Production Config
```toml
level = "INFO"
global_console_display = false
global_file_storage = true
debug_mode = false
```

### Testing Config
```toml
level = "DEBUG"
global_console_display = true
global_file_storage = false
```

## Best Practices

1. **Version Control**: Add `logly.toml` to `.gitignore` for environment-specific configs
2. **Template**: Provide `logly.toml.example` in repository
3. **Documentation**: Document all config options used
4. **Validation**: Test config file before deployment
5. **Overrides**: Use manual config for runtime changes

## Error Handling

### File Not Found
```rust
// Custom path - returns error if not found
let logger = Logger::with_config_file(PathBuf::from("missing.toml"))?;
```

### Invalid TOML
```
Error: Failed to parse config file: expected a table key, found a newline at line 5
```

### Invalid Values
```
Error: Invalid log level: INVALID
```

## Performance

- Config file loaded once at initialization
- No performance impact after loading
- Scanning can be disabled for faster startup
- Manual configuration has no file I/O overhead
