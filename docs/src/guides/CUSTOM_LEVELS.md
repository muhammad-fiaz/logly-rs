# Custom Log Levels

Add your own log levels with custom priorities and colors.

## Overview

Beyond the 8 built-in levels, you can create custom levels for specific use cases.

## Adding Custom Levels

```rust
use logly::prelude::*;

let logger = Logger::new();
logger.add_sink(SinkConfig::default())?;

// Add custom level: name, priority, ANSI color code
logger.add_custom_level("NOTICE".to_string(), 35, "96".to_string())?;
logger.add_custom_level("AUDIT".to_string(), 42, "95".to_string())?;
```

## Using Custom Levels

```rust
logger.log_custom("NOTICE", "Important notice".to_string())?;
logger.log_custom("AUDIT", "Security audit log".to_string())?;
```

## Priority System

Custom levels fit into the priority hierarchy:

| Level | Priority |
|-------|----------|
| TRACE | 5 |
| DEBUG | 10 |
| INFO | 20 |
| SUCCESS | 25 |
| WARNING | 30 |
| **NOTICE** | **35** (custom) |
| ERROR | 40 |
| **AUDIT** | **42** (custom) |
| FAIL | 45 |
| CRITICAL | 50 |

## ANSI Color Codes

Common color codes:
- `30` - Black
- `31` - Red
- `32` - Green
- `33` - Yellow
- `34` - Blue
- `35` - Magenta
- `36` - Cyan
- `37` - White
- `90-97` - Bright colors

## Examples

### Security Audit Level

```rust
logger.add_custom_level("AUDIT".to_string(), 42, "95".to_string())?;
logger.log_custom("AUDIT", "User login attempt".to_string())?;
```

### Performance Metrics

```rust
logger.add_custom_level("METRIC".to_string(), 15, "36".to_string())?;
logger.log_custom("METRIC", "Response time: 45ms".to_string())?;
```

### Business Events

```rust
logger.add_custom_level("BUSINESS".to_string(), 28, "33".to_string())?;
logger.log_custom("BUSINESS", "Order placed: $99.99".to_string())?;
```

## See Also

- [Log Levels](./LEVELS.md)
- [Formatting](./FORMATTING.md)
