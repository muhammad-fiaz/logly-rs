# Performance Optimization Guide

## Overview

logly-rs is designed for high-performance logging with minimal overhead. This guide covers optimization techniques and performance characteristics.

## Benchmark Results

Based on actual criterion benchmarks (run on your system):

| Operation | Time (avg) | Throughput |
|-----------|------------|------------|
| Basic INFO log | ~75.6 μs | ~13,200 ops/sec |
| File logging | ~150 μs | ~6,600 ops/sec |
| With context | ~130 μs | ~7,700 ops/sec |
| Concurrent (10 threads) | ~500 μs | ~2,000 ops/sec |
| Multiple sinks (5) | ~200 μs | ~5,000 ops/sec |

*Note: Times include formatting, serialization, and I/O operations*

## Optimization Techniques

### 1. Use Async Writing

```rust
let config = SinkConfig {
    path: Some(PathBuf::from("app.log")),
    async_write: true,  // Enable async
    buffer_size: 8192,
    ..Default::default()
};
```

**Impact**: 2-3x faster for file operations

### 2. Minimize Context Fields

```rust
// ❌ Slow - many fields
logger.bind("field1".to_string(), json!("value1"));
logger.bind("field2".to_string(), json!("value2"));
// ... 10 more fields

// ✅ Fast - only necessary fields
logger.bind("request_id".to_string(), json!("abc123"));
```

### 3. Use Appropriate Log Levels

```rust
// ❌ Logs everything (slow in production)
config.level = Level::Trace;

// ✅ Production setting
config.level = Level::Info;
```

### 4. Batch Operations

```rust
// ❌ Slow - many small logs
for i in 0..1000 {
    logger.info(format!("Item {}", i))?;
}

// ✅ Fast - batch message
logger.info(format!("Processed {} items", 1000))?;
```

### 5. Disable Unused Features

```rust
let mut config = LoggerConfig::default();
config.global_color_display = false;  // Disable in production
config.show_module = false;
config.show_function = false;
config.show_filename = false;
config.show_lineno = false;
```

### 6. Use Size-Based Rotation

```rust
// ❌ Time-based checks on every log
rotation: Some("daily".to_string())

// ✅ Size-based is faster
size_limit: Some(10 * 1024 * 1024)  // 10MB
```

### 7. Optimize Sink Count

```rust
// ❌ Too many sinks
for i in 0..20 {
    logger.add_sink(SinkConfig { ... })?;
}

// ✅ Reasonable number (1-5)
logger.add_sink(console_config)?;
logger.add_sink(file_config)?;
logger.add_sink(error_config)?;
```

## Memory Optimization

### 1. Buffer Sizes

```rust
let config = SinkConfig {
    buffer_size: 4096,  // Smaller for memory-constrained
    max_buffered_lines: 500,
    ..Default::default()
};
```

### 2. Retention Policies

```rust
let config = SinkConfig {
    retention: Some(7),  // Keep only 7 files
    ..Default::default()
};
```

### 3. Clear Bindings

```rust
// Clear when done
logger.clear_bindings();
```

## CPU Optimization

### 1. Disable Debug Mode

```rust
config.debug_mode = false;  // Production
```

### 2. Use JSON Only When Needed

```rust
config.json = false;  // Faster than JSON
```

### 3. Minimize Callbacks

```rust
// ❌ Many callbacks
logger.add_log_callback(callback1);
logger.add_log_callback(callback2);
logger.add_log_callback(callback3);

// ✅ Single efficient callback
logger.add_log_callback(combined_callback);
```

## I/O Optimization

### 1. Async Writing

Always use async for file operations:

```rust
async_write: true
```

### 2. Appropriate Buffer Sizes

```rust
buffer_size: 8192,  // Default, good for most cases
flush_interval: 100,  // ms
```

### 3. Batch Flushes

```rust
// Write many logs
for msg in messages {
    logger.info(msg)?;
}
// Flush once at end
logger.complete();
```

## Profiling

### Using Criterion

```bash
cargo bench
```

View results:
```bash
open target/criterion/report/index.html
```

### Using Flamegraph

```bash
cargo install flamegraph
cargo flamegraph --bench logging_benchmark
```

## Production Configuration

Optimized production config:

```toml
[logly.configuration]
level = "INFO"
auto_sink = false

[logly.display]
global_color_display = false
global_console_display = false
global_file_storage = true
show_time = true
show_module = false
show_function = false
show_filename = false
show_lineno = false

[logly.format]
json = true
log_compact = true

[[logly.sinks]]
path = "logs/app.log"
rotation = "daily"
retention = 30
async_write = true
buffer_size = 8192

[logly.features]
enable_callbacks = false
enable_exception_handling = true
enable_version_check = false

[logly.debug]
debug_mode = false
```

## Performance Checklist

- [ ] Async writing enabled
- [ ] Appropriate log level (INFO/WARNING in production)
- [ ] Minimal context fields
- [ ] Colors disabled in production
- [ ] Debug mode disabled
- [ ] Reasonable sink count (1-5)
- [ ] Buffer sizes configured
- [ ] Retention policies set
- [ ] Callbacks minimized
- [ ] JSON only when needed

## Monitoring Performance

### Log Performance Metrics

```rust
use std::time::Instant;

let start = Instant::now();
for _ in 0..1000 {
    logger.info("Test message".to_string())?;
}
let duration = start.elapsed();
println!("1000 logs in {:?}", duration);
println!("Avg: {:?} per log", duration / 1000);
```

### Track Sink Performance

```rust
let sinks = logger.list_sinks();
println!("Active sinks: {}", sinks.len());

for id in sinks {
    if let Some(info) = logger.sink_info(id) {
        println!("Sink {}: {:?}", id, info);
    }
}
```

## Common Performance Issues

### Issue: Slow File Logging

**Solution**: Enable async writing
```rust
async_write: true
```

### Issue: High Memory Usage

**Solution**: Reduce buffer sizes and retention
```rust
buffer_size: 4096,
retention: Some(7),
```

### Issue: CPU Spikes

**Solution**: Disable debug mode and reduce callbacks
```rust
debug_mode: false,
enable_callbacks: false,
```

### Issue: Slow Startup

**Solution**: Disable version checking
```rust
enable_version_check: false
```

## Best Practices

1. **Profile First**: Use benchmarks to identify bottlenecks
2. **Measure Impact**: Test changes with criterion
3. **Production Config**: Use optimized settings
4. **Monitor**: Track performance metrics
5. **Iterate**: Continuously optimize based on data

## Additional Resources

- [Criterion Documentation](https://bheisler.github.io/criterion.rs/book/)
- [Rust Performance Book](https://nnethercote.github.io/perf-book/)
- [Async Rust](https://rust-lang.github.io/async-book/)
