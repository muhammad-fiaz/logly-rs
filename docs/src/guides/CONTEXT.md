# Context Binding

Context binding allows you to attach persistent key-value pairs to all log messages.

## Overview

Context fields are automatically included in every log message, making it easy to track requests, users, sessions, and other contextual information.

## Basic Usage

### Bind Context

```rust
use logly::prelude::*;

let logger = Logger::new();
logger.add_sink(SinkConfig::default())?;

// Bind context fields
logger.bind("user_id".to_string(), serde_json::json!("12345"));
logger.bind("session".to_string(), serde_json::json!("abc-def-ghi"));
logger.bind("request_id".to_string(), serde_json::json!("req-001"));

// All logs now include these fields
logger.info("User logged in".to_string())?;
```

### Unbind Context

```rust
// Remove specific field
logger.unbind("session");

// Clear all context
logger.clear_bindings();
```

## Use Cases

### Request Tracking

```rust
fn handle_request(logger: &Logger, request_id: &str) -> Result<(), Box<dyn std::error::Error>> {
    logger.bind("request_id".to_string(), serde_json::json!(request_id));
    
    logger.info("Processing request".to_string())?;
    // ... handle request ...
    logger.success("Request completed".to_string())?;
    
    logger.unbind("request_id");
    Ok(())
}
```

### User Context

```rust
fn user_action(logger: &Logger, user_id: u64, action: &str) -> Result<(), Box<dyn std::error::Error>> {
    logger.bind("user_id".to_string(), serde_json::json!(user_id));
    logger.bind("action".to_string(), serde_json::json!(action));
    
    logger.info(format!("User performed: {}", action))?;
    
    logger.clear_bindings();
    Ok(())
}
```

### Application Context

```rust
// Set at startup
logger.bind("app_name".to_string(), serde_json::json!("myapp"));
logger.bind("version".to_string(), serde_json::json!("1.0.0"));
logger.bind("environment".to_string(), serde_json::json!("production"));

// These fields appear in all logs throughout the application
```

## JSON Output

Context fields are especially useful with JSON logging:

```rust
let mut config = LoggerConfig::default();
config.json = true;
logger.configure(config);

logger.bind("user_id".to_string(), serde_json::json!("12345"));
logger.info("Action performed".to_string())?;
```

Output:
```json
{
  "timestamp": "2024-01-01T12:00:00.000Z",
  "level": "INFO",
  "message": "Action performed",
  "user_id": "12345"
}
```

## Complex Values

Bind any JSON-serializable value:

```rust
use serde_json::json;

// Objects
logger.bind("user".to_string(), json!({
    "id": 12345,
    "name": "John Doe",
    "role": "admin"
}));

// Arrays
logger.bind("tags".to_string(), json!(["important", "urgent"]));

// Numbers
logger.bind("retry_count".to_string(), json!(3));

// Booleans
logger.bind("is_authenticated".to_string(), json!(true));
```

## Thread Safety

Context bindings are thread-safe and can be used across threads:

```rust
use std::sync::Arc;
use std::thread;

let logger = Arc::new(Logger::new());
logger.add_sink(SinkConfig::default())?;

logger.bind("app".to_string(), serde_json::json!("myapp"));

let logger_clone = Arc::clone(&logger);
thread::spawn(move || {
    logger_clone.info("From thread".to_string()).unwrap();
    // Still includes "app" context
});
```

## Best Practices

1. **Bind early**: Set application-wide context at startup
2. **Unbind when done**: Remove request-specific context after handling
3. **Use meaningful keys**: Choose clear, consistent key names
4. **Avoid sensitive data**: Don't bind passwords or tokens
5. **Keep it simple**: Don't overload with too many fields

## Example: Web Server

```rust
use logly::prelude::*;
use std::sync::Arc;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let logger = Arc::new(Logger::new());
    logger.add_sink(SinkConfig::default())?;
    
    // Application context
    logger.bind("app".to_string(), serde_json::json!("web-server"));
    logger.bind("version".to_string(), serde_json::json!("1.0.0"));
    
    logger.info("Server starting".to_string())?;
    
    // Simulate request handling
    handle_request(&logger, "req-001", "user-123")?;
    
    Ok(())
}

fn handle_request(logger: &Logger, request_id: &str, user_id: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Request-specific context
    logger.bind("request_id".to_string(), serde_json::json!(request_id));
    logger.bind("user_id".to_string(), serde_json::json!(user_id));
    
    logger.info("Request received".to_string())?;
    logger.debug("Processing...".to_string())?;
    logger.success("Request completed".to_string())?;
    
    // Clean up request context
    logger.unbind("request_id");
    logger.unbind("user_id");
    
    Ok(())
}
```

## See Also

- [JSON Logging](./FORMATTING.md)
- [Configuration](../CONFIGURATION.md)
- [Quick Start](../QUICK_START.md)
