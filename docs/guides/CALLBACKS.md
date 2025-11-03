# Callbacks Guide

## Overview

Logly provides three types of callbacks for custom behavior:
- **Log Callbacks**: Monitor log events
- **Color Callbacks**: Custom color formatting
- **Exception Callbacks**: Error handling hooks

## Log Callbacks

Monitor and react to log events:

```rust
logger.add_log_callback(|record| {
    if record.level >= Level::Error {
        // Send alert, update metrics, etc.
        println!("High severity: {}", record.message);
    }
    Ok(())
});
```

### Use Cases
- Metrics collection
- Alert triggering
- Audit logging
- Real-time monitoring

## Color Callbacks

Customize color output:

```rust
logger.add_color_callback(|level, message| {
    match level {
        Level::Error => format!("\x1b[91;1m{}\x1b[0m", message),  // Bright red bold
        Level::Warning => format!("\x1b[93m{}\x1b[0m", message),  // Yellow
        Level::Success => format!("\x1b[32;1m{}\x1b[0m", message), // Green bold
        _ => message.to_string(),
    }
});
```

### ANSI Color Codes
- `30-37`: Standard colors
- `90-97`: Bright colors
- `1`: Bold
- `4`: Underline
- `91;1`: Bright red bold

## Exception Callbacks

Handle errors and exceptions:

```rust
logger.add_exception_callback(|error, backtrace| {
    eprintln!("⚠️  EXCEPTION: {}", error);
    eprintln!("Backtrace:\n{}", backtrace);
    
    // Send to error tracking service
    // Log to file
    // Trigger alerts
});
```

## Complete Example

```rust
use logly::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let logger = Logger::new();
    
    // Log callback for monitoring
    logger.add_log_callback(|record| {
        if record.level >= Level::Error {
            println!("[MONITOR] High severity: {}", record.message);
        }
        Ok(())
    });
    
    // Color callback for custom formatting
    logger.add_color_callback(|level, message| {
        let color = match level {
            Level::Trace => "36",
            Level::Debug => "34",
            Level::Info => "37",
            Level::Success => "32;1",
            Level::Warning => "33",
            Level::Error => "31",
            Level::Fail => "35",
            Level::Critical => "91;1",
        };
        format!("\x1b[{}m● {}\x1b[0m", color, message)
    });
    
    // Exception callback
    logger.add_exception_callback(|error, backtrace| {
        eprintln!("\n⚠️  EXCEPTION CAUGHT");
        eprintln!("Error: {}", error);
        eprintln!("Backtrace:\n{}", backtrace);
    });
    
    logger.add_sink(SinkConfig::default())?;
    
    logger.info("Info message".to_string())?;
    logger.error("Error message".to_string())?;
    logger.success("Success message".to_string())?;
    
    Ok(())
}
```

## Clearing Callbacks

```rust
logger.clear_callbacks();  // Remove all callbacks
```

## Best Practices

1. **Keep callbacks fast**: Avoid blocking operations
2. **Handle errors**: Return `Ok(())` or error string
3. **Thread safety**: Callbacks are executed in logging thread
4. **Performance**: Minimize callback overhead
