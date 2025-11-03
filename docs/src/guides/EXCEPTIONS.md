# Exception Handling

Comprehensive error handling with backtraces.

## Enable Exception Handling

```rust
let mut config = LoggerConfig::default();
config.enable_exception_handling = true;
logger.configure(config);
```

## Exception Callback

Handle exceptions with custom logic:

```rust
logger.add_exception_callback(|error, backtrace| {
    eprintln!("Exception occurred: {}", error);
    eprintln!("Backtrace:\n{}", backtrace);
    
    // Send to monitoring service
    // alert_team(&error);
});
```

## Error Types

Logly uses `LoglyError` enum:
- `IoError` - File I/O errors
- `ConfigError` - Configuration errors
- `SinkError` - Sink operation errors
- `GpuError` - GPU-related errors
- `FormatError` - Formatting errors

## Example

```rust
use logly::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let logger = Logger::new();
    
    logger.add_exception_callback(|error, backtrace| {
        eprintln!("Error: {}\n{}", error, backtrace);
    });
    
    logger.add_sink(SinkConfig::default())?;
    logger.info("Running".to_string())?;
    
    Ok(())
}
```

## See Also

- [Callbacks](./CALLBACKS.md)
- [Troubleshooting](./TROUBLESHOOTING.md)
