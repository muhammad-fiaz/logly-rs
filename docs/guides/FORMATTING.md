# Template String Formatting Guide

## Overview

Logly supports custom format strings with placeholders for complete control over log output.

## Built-in Placeholders

- `{time}` - Timestamp (RFC3339 format)
- `{time:PATTERN}` - Custom time format
- `{level}` - Log level name
- `{message}` - Log message
- `{module}` - Module name
- `{function}` - Function name
- `{filename}` - Source filename
- `{lineno}` - Line number
- `{FIELD}` - Any custom field

## Time Format Patterns

### Supported Patterns

| Pattern | Description | Example |
|---------|-------------|---------|
| YYYY | 4-digit year | 2025 |
| YY | 2-digit year | 25 |
| MMMM | Full month name | October |
| MMM | Abbreviated month | Oct |
| MM | 2-digit month | 10 |
| dddd | Full weekday name | Saturday |
| ddd | Abbreviated weekday | Sat |
| DD | 2-digit day | 11 |
| HH | 2-digit hour (24h) | 13 |
| hh | 2-digit hour (12h) | 01 |
| mm | 2-digit minute | 45 |
| ss | 2-digit second | 32 |
| SSS | Milliseconds | 123 |
| SSSSSS | Microseconds | 123456 |
| A | AM/PM | AM |
| a | am/pm | am |
| ZZ | Timezone (+00:00) | +00:00 |
| Z | Timezone (+0000) | +0000 |

## Examples

### Date Only
```rust
SinkConfig {
    format: Some("{time:YYYY-MM-DD} | {level} | {message}".to_string()),
    ..Default::default()
}
// Output: 2025-10-11 | INFO | Message
```

### Full DateTime
```rust
SinkConfig {
    format: Some("{time:YYYY-MM-DD HH:mm:ss} [{level}] {message}".to_string()),
    ..Default::default()
}
// Output: 2025-10-11 13:45:32 [INFO] Message
```

### With Milliseconds
```rust
SinkConfig {
    format: Some("{time:HH:mm:ss.SSS} | {message}".to_string()),
    ..Default::default()
}
// Output: 13:45:32.123 | Message
```

### ISO 8601 Style
```rust
SinkConfig {
    format: Some("{time:YYYY-MM-DDTHH:mm:ss} {level} {message}".to_string()),
    ..Default::default()
}
// Output: 2025-10-11T13:45:32 INFO Message
```

### Readable Format
```rust
SinkConfig {
    format: Some("{time:dddd, DD MMMM YYYY at HH:mm:ss} - {level} - {message}".to_string()),
    ..Default::default()
}
// Output: Saturday, 11 October 2025 at 13:45:32 - INFO - Message
```

### European Format
```rust
SinkConfig {
    format: Some("{time:DD/MM/YYYY HH:mm} {message}".to_string()),
    ..Default::default()
}
// Output: 11/10/2025 13:45 Message
```

## Common Patterns

### Simple Console
```rust
format: Some("{time:HH:mm:ss} [{level}] {message}".to_string())
```

### Detailed File
```rust
format: Some("{time:YYYY-MM-DD HH:mm:ss.SSS} | {level:8} | {module}:{function} | {message}".to_string())
```

### Request Logging
```rust
format: Some("{time:YYYY-MM-DD HH:mm:ss} | {method} {path} | status={status_code} | duration={duration_ms}ms".to_string())
```

## Best Practices

1. **Console vs File**: Use simpler formats for console, detailed for files
2. **Time Formats**: 
   - `HH:mm:ss` for console (quick reference)
   - `YYYY-MM-DD HH:mm:ss.SSS` for logs (precise timestamps)
   - `YYYY-MM-DDTHH:mm:ss` for ISO 8601 compatibility
3. **Performance**: Simpler formats are faster to process
4. **Consistency**: Use consistent formats across your application
