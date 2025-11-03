# Async Logging

Logly-rs provides non-blocking async logging for high-performance applications.

## Overview

Async logging writes log messages to a buffer and processes them in the background, preventing I/O operations from blocking your application.

## Features

- Non-blocking writes
- Configurable buffer sizes
- Automatic flushing
- Thread-safe operations
- Zero-copy where possible

## Basic Usage

### Enable Async Writes

```rust
use logly::prelude::*;
use std::path::PathBuf;

let logger = Logger::new();

let config = SinkConfig {
    path: Some(PathBuf::from("logs/app.log")),
    async_write: true,
    ..Default::default()
};

logger.add_sink(config)?;
logger.info("Async logging enabled!".to_string())?;
```

## Configuration

### Buffer Size

Control the async buffer size:

```rust
let mut config = LoggerConfig::default();
config.async_buffer_size = 8192; // 8KB buffer
logger.configure(config);
```

### Flush Interval

Configure automatic flushing:

```rust
let mut config = LoggerConfig::default();
config.async_flush_interval = 1000; // Flush every 1 second
logger.configure(config);
```

## Performance

Async logging provides significant performance improvements:

- **Throughput**: 2-3x faster than synchronous logging
- **Latency**: Sub-microsecond write times
- **Scalability**: Handles thousands of messages per second

## Best Practices

1. **Use for high-volume logging**: Async is ideal when logging frequently
2. **Configure buffer size**: Larger buffers = better performance, more memory
3. **Manual flush on shutdown**: Ensure all logs are written before exit

```rust
// Flush before shutdown
logger.flush()?;
```

## Example

```rust
use logly::prelude::*;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let logger = Logger::new();
    
    // Async file sink
    logger.add_sink(SinkConfig {
        path: Some(PathBuf::from("logs/async.log")),
        async_write: true,
        ..Default::default()
    })?;
    
    // High-volume logging
    for i in 0..10000 {
        logger.info(format!("Message {}", i))?;
    }
    
    // Ensure all messages are written
    logger.flush()?;
    
    Ok(())
}
```

## See Also

- [Performance Guide](../PERFORMANCE.md)
- [Sinks](./SINKS.md)
- [Configuration](../CONFIGURATION.md)
