# File Rotation & Retention Guide

## Rotation Intervals

Logly supports time-based rotation:

- **hourly** - Rotate every hour
- **daily** - Rotate every day
- **weekly** - Rotate every week
- **monthly** - Rotate every 30 days
- **yearly** - Rotate every 365 days

## Time-Based Rotation

```rust
SinkConfig {
    path: Some(PathBuf::from("logs/app.log")),
    rotation: Some("daily".to_string()),
    retention: Some(7),  // Keep 7 days
    ..Default::default()
}
```

## Size-Based Rotation

```rust
SinkConfig {
    path: Some(PathBuf::from("logs/app.log")),
    size_limit: Some(10 * 1024 * 1024),  // 10MB
    retention: Some(5),  // Keep 5 files
    ..Default::default()
}
```

## Combined Rotation

Rotate when EITHER condition is met:

```rust
SinkConfig {
    path: Some(PathBuf::from("logs/app.log")),
    rotation: Some("daily".to_string()),
    size_limit: Some(10 * 1024 * 1024),  // Daily OR 10MB
    retention: Some(7),
    ..Default::default()
}
```

## Retention Policies

Automatically delete old log files:

```rust
SinkConfig {
    retention: Some(10),  // Keep only 10 most recent files
    ..Default::default()
}
```

## Rotated File Naming

Files are renamed with timestamps:
```
app.log -> app_20240101_120000.log
```

## Examples

### Hourly Rotation
```rust
SinkConfig {
    path: Some(PathBuf::from("logs/hourly.log")),
    rotation: Some("hourly".to_string()),
    retention: Some(24),  // Keep 24 hours
    ..Default::default()
}
```

### Daily Rotation
```rust
SinkConfig {
    path: Some(PathBuf::from("logs/daily.log")),
    rotation: Some("daily".to_string()),
    retention: Some(7),  // Keep 7 days
    ..Default::default()
}
```

### Weekly Rotation
```rust
SinkConfig {
    path: Some(PathBuf::from("logs/weekly.log")),
    rotation: Some("weekly".to_string()),
    retention: Some(4),  // Keep 4 weeks
    ..Default::default()
}
```

### Monthly Rotation
```rust
SinkConfig {
    path: Some(PathBuf::from("logs/monthly.log")),
    rotation: Some("monthly".to_string()),
    retention: Some(12),  // Keep 12 months
    ..Default::default()
}
```

### Yearly Rotation
```rust
SinkConfig {
    path: Some(PathBuf::from("logs/yearly.log")),
    rotation: Some("yearly".to_string()),
    retention: Some(5),  // Keep 5 years
    ..Default::default()
}
```

### Size-Based (100MB)
```rust
SinkConfig {
    path: Some(PathBuf::from("logs/app.log")),
    size_limit: Some(100 * 1024 * 1024),
    retention: Some(10),
    ..Default::default()
}
```

## Best Practices

1. **Development**: Hourly or daily rotation
2. **Production**: Daily or weekly rotation
3. **Archives**: Monthly or yearly rotation
4. **High-Volume**: Size-based rotation
5. **Compliance**: Long retention periods

## Automatic Cleanup

Old files are automatically deleted based on retention policy:
- Sorted by modification time
- Oldest files deleted first
- Happens during rotation
