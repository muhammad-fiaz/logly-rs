# JSON Logging

Structured JSON output for log aggregation and analysis.

## Enable JSON

```rust
let mut config = LoggerConfig::default();
config.json = true;
logger.configure(config);

logger.add_sink(SinkConfig::default())?;
logger.info("JSON message".to_string())?;
```

## Output Format

```json
{
  "timestamp": "2024-01-01T12:00:00.000Z",
  "level": "INFO",
  "message": "JSON message",
  "module": "myapp",
  "function": "main",
  "line": 42
}
```

## With Context

```rust
logger.bind("user_id".to_string(), serde_json::json!("12345"));
logger.bind("request_id".to_string(), serde_json::json!("req-001"));

logger.info("User action".to_string())?;
```

Output:
```json
{
  "timestamp": "2024-01-01T12:00:00.000Z",
  "level": "INFO",
  "message": "User action",
  "user_id": "12345",
  "request_id": "req-001"
}
```

## Use Cases

### Log Aggregation
- Elasticsearch
- Splunk
- CloudWatch
- Datadog

### Analysis
- Parse with jq
- Import to databases
- Process with scripts

## Example

```rust
use logly::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let logger = Logger::new();
    
    let mut config = LoggerConfig::default();
    config.json = true;
    logger.configure(config);
    
    logger.add_sink(SinkConfig::default())?;
    
    logger.bind("app".to_string(), serde_json::json!("myapp"));
    logger.info("Application started".to_string())?;
    
    Ok(())
}
```

## See Also

- [Context Binding](./CONTEXT.md)
- [Formatting](./FORMATTING.md)
