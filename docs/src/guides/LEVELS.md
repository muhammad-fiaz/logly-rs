# Log Levels Guide

## Standard Log Levels

Logly provides 8 standard log levels with specific priorities:

| Level    | Priority | Method | Use Case |
|----------|----------|--------|----------|
| TRACE    | 5        | `logger.trace()` | Very detailed debugging information |
| DEBUG    | 10       | `logger.debug()` | Debugging information for developers |
| INFO     | 20       | `logger.info()` | General informational messages |
| SUCCESS  | 25       | `logger.success()` | Successful operation completion |
| WARNING  | 30       | `logger.warning()` | Warning messages that need attention |
| ERROR    | 40       | `logger.error()` | Error conditions that need fixing |
| FAIL     | 45       | `logger.fail()` | Operation failures |
| CRITICAL | 50       | `logger.critical()` | Critical system errors |

## Usage

```rust
use logly::prelude::*;

let logger = Logger::new();

logger.trace("Detailed trace information".to_string())?;
logger.debug("Debug information".to_string())?;
logger.info("Application started".to_string())?;
logger.success("Operation completed successfully!".to_string())?;
logger.warning("Warning message".to_string())?;
logger.error("Error occurred".to_string())?;
logger.fail("Operation failed".to_string())?;
logger.critical("Critical system error!".to_string())?;
```

## Level Filtering

Set minimum log level to filter messages:

```rust
let mut config = LoggerConfig::default();
config.level = Level::Warning;  // Only WARNING and above
logger.configure(config);

logger.debug("Not shown".to_string())?;
logger.warning("Shown".to_string())?;
logger.error("Shown".to_string())?;
```

## Custom Log Levels

Add your own log levels with custom priorities and colors:

```rust
// Add custom level between WARNING (30) and ERROR (40)
logger.add_custom_level("NOTICE".to_string(), 35, "96".to_string())?;

// Use custom level
logger.log_custom("NOTICE", "Custom level message".to_string())?;

// Remove custom level
logger.remove_custom_level("NOTICE");
```

### Custom Level Priority Rules

- Priority must be between 0-255
- Higher priority = more severe
- Custom levels integrate with standard filtering
- Duplicate names are rejected

### Custom Level Colors

ANSI color codes:
- `"30-37"` - Standard colors
- `"90-97"` - Bright colors
- `"1"` - Bold
- `"4"` - Underline

Example:
```rust
logger.add_custom_level("AUDIT".to_string(), 28, "93".to_string())?;  // Bright yellow
logger.add_custom_level("SECURITY".to_string(), 48, "91;1".to_string())?;  // Bright red bold
```

## Level Comparison

Levels can be compared:

```rust
assert!(Level::Trace < Level::Debug);
assert!(Level::Debug < Level::Info);
assert!(Level::Error > Level::Warning);

if record.level >= Level::Error {
    // Handle high severity
}
```

## Level Priority Access

```rust
let priority = Level::Info.priority();  // Returns 20
let level = Level::from_priority(40);   // Returns Some(Level::Error)
```

## Per-Level Configuration

Configure display options per level:

```rust
let mut config = LoggerConfig::default();

// Disable console for DEBUG level
config.console_levels.insert(Level::Debug, false);

// Disable colors for ERROR level
config.color_levels.insert(Level::Error, false);

// Disable timestamp for TRACE level
config.time_levels.insert(Level::Trace, false);

logger.configure(config);
```

## Level Colors

Default colors:
- TRACE: Cyan (36)
- DEBUG: Blue (34)
- INFO: White (37)
- SUCCESS: Green (32)
- WARNING: Yellow (33)
- ERROR: Red (31)
- FAIL: Magenta (35)
- CRITICAL: Bright Red (91)

Customize colors:

```rust
let mut config = LoggerConfig::default();
config.level_colors.insert(Level::Info, "92".to_string());  // Bright green
config.level_colors.insert(Level::Error, "91;1".to_string());  // Bright red bold
logger.configure(config);
```

## Best Practices

1. **TRACE**: Use for very detailed debugging, disabled in production
2. **DEBUG**: Development debugging, disabled in production
3. **INFO**: General information, enabled in production
4. **SUCCESS**: Highlight successful operations
5. **WARNING**: Potential issues that don't stop execution
6. **ERROR**: Errors that need attention but don't crash
7. **FAIL**: Operation failures that may need retry
8. **CRITICAL**: Severe errors that may crash the application

## Examples

### Development Logging
```rust
config.level = Level::Trace;  // Show everything
```

### Production Logging
```rust
config.level = Level::Info;  // Hide TRACE and DEBUG
```

### Error-Only Logging
```rust
config.level = Level::Error;  // Only errors and critical
```

### Custom Audit Level
```rust
logger.add_custom_level("AUDIT".to_string(), 28, "93".to_string())?;
logger.log_custom("AUDIT", "User login successful".to_string())?;
```
