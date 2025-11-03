# Auto-Sink

Automatic sink initialization for quick setup.

## Overview

Auto-sink automatically creates a console sink when the logger is initialized, allowing immediate logging without manual sink setup.

## Enabled by Default

```rust
use logly::prelude::*;

let logger = Logger::new();
// Auto-sink creates console sink automatically

logger.info("Works immediately!".to_string())?;
```

## Disable Auto-Sink

```rust
let mut config = LoggerConfig::default();
config.auto_sink = false;
logger.configure(config);

// Must manually add sinks
logger.add_sink(SinkConfig::default())?;
```

## When to Disable

Disable auto-sink when you want full control:

```rust
let mut config = LoggerConfig::default();
config.auto_sink = false;
logger.configure(config);

// Add only file sink, no console
logger.add_sink(SinkConfig {
    path: Some(PathBuf::from("app.log")),
    ..Default::default()
})?;
```

## See Also

- [Sinks](./SINKS.md)
- [Configuration](../CONFIGURATION.md)
