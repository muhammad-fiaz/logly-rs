# Filtering

Control which log messages are processed and displayed.

## Overview

Filtering allows you to:
- Set minimum log levels
- Filter by module or function
- Create custom filters
- Control output per sink

## Level Filtering

### Global Level

Set minimum level for all sinks:

```rust
use logly::prelude::*;

let logger = Logger::new();

let mut config = LoggerConfig::default();
config.level = Level::Warning; // Only WARNING and above
logger.configure(config);

logger.add_sink(SinkConfig::default())?;

logger.debug("Not shown".to_string())?;
logger.warning("Shown!".to_string())?;
```

### Per-Sink Level

Different levels for different sinks:

```rust
use std::path::PathBuf;

// Console: INFO and above
logger.add_sink(SinkConfig {
    level: Some(Level::Info),
    ..Default::default()
})?;

// File: DEBUG and above
logger.add_sink(SinkConfig {
    path: Some(PathBuf::from("debug.log")),
    level: Some(Level::Debug),
    ..Default::default()
})?;

// Errors only
logger.add_sink(SinkConfig {
    path: Some(PathBuf::from("errors.log")),
    level: Some(Level::Error),
    ..Default::default()
})?;
```

## Module Filtering

Filter by module name:

```rust
let mut config = LoggerConfig::default();
config.module_filter = Some(vec![
    "myapp::api".to_string(),
    "myapp::database".to_string(),
]);
logger.configure(config);
```

## Function Filtering

Filter by function name:

```rust
let mut config = LoggerConfig::default();
config.function_filter = Some(vec![
    "handle_request".to_string(),
    "process_data".to_string(),
]);
logger.configure(config);
```

## Level Priorities

Log levels and their priorities:

| Level | Priority | Description |
|-------|----------|-------------|
| TRACE | 5 | Very detailed debugging |
| DEBUG | 10 | Debugging information |
| INFO | 20 | General information |
| SUCCESS | 25 | Success messages |
| WARNING | 30 | Warning messages |
| ERROR | 40 | Error messages |
| FAIL | 45 | Failure messages |
| CRITICAL | 50 | Critical errors |

## Examples

### Development vs Production

```rust
#[cfg(debug_assertions)]
let level = Level::Debug;

#[cfg(not(debug_assertions))]
let level = Level::Info;

let mut config = LoggerConfig::default();
config.level = level;
logger.configure(config);
```

### Separate Error Logs

```rust
use std::path::PathBuf;

let logger = Logger::new();

// All logs to main file
logger.add_sink(SinkConfig {
    path: Some(PathBuf::from("app.log")),
    level: Some(Level::Debug),
    ..Default::default()
})?;

// Errors to separate file
logger.add_sink(SinkConfig {
    path: Some(PathBuf::from("errors.log")),
    level: Some(Level::Error),
    ..Default::default()
})?;

logger.debug("Debug message".to_string())?; // Only in app.log
logger.error("Error message".to_string())?; // In both files
```

### Dynamic Level Changes

```rust
// Start with INFO
let mut config = LoggerConfig::default();
config.level = Level::Info;
logger.configure(config);

// ... later, enable debug mode
config.level = Level::Debug;
logger.configure(config);
```

## Best Practices

1. **Production**: Use INFO or WARNING level
2. **Development**: Use DEBUG or TRACE level
3. **Errors**: Always log ERROR and above
4. **Performance**: Higher levels = better performance
5. **Separate files**: Use different sinks for different levels

## See Also

- [Log Levels](./LEVELS.md)
- [Sinks](./SINKS.md)
- [Configuration](../CONFIGURATION.md)
