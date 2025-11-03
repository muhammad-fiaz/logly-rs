# Debug Mode

Enable detailed internal logging for troubleshooting.

## Enable Debug Mode

```rust
use logly::prelude::*;

let logger = Logger::new();

let mut config = LoggerConfig::default();
config.debug_mode = true;
logger.configure(config);
```

## Debug to File

```rust
use std::path::PathBuf;

let mut config = LoggerConfig::default();
config.debug_mode = true;
config.debug_log_file = Some(PathBuf::from("logly_debug.log"));
logger.configure(config);
```

## What Gets Logged

Debug mode logs internal operations:
- Sink initialization
- Configuration changes
- Rotation events
- GPU operations
- Callback execution
- Error handling

## Example Output

```
[LOGLY DEBUG] Initializing logger
[LOGLY DEBUG] Adding sink: console
[LOGLY DEBUG] Configuration updated: level=DEBUG
[LOGLY DEBUG] Rotation triggered: size limit reached
[LOGLY DEBUG] Rotated file: app_20240101_120000.log
```

## Use Cases

### Troubleshooting

```rust
// Enable debug to diagnose issues
config.debug_mode = true;
config.debug_log_file = Some(PathBuf::from("debug.log"));
logger.configure(config);

// Your logging code
logger.info("Test".to_string())?;

// Check debug.log for internal details
```

### Performance Analysis

```rust
config.debug_mode = true;
logger.configure(config);

// Logs timing information for operations
```

### Development

```rust
#[cfg(debug_assertions)]
{
    config.debug_mode = true;
    logger.configure(config);
}
```

## Disable in Production

```rust
#[cfg(not(debug_assertions))]
{
    config.debug_mode = false;
    logger.configure(config);
}
```

## See Also

- [Troubleshooting](./TROUBLESHOOTING.md)
- [Configuration](../CONFIGURATION.md)
