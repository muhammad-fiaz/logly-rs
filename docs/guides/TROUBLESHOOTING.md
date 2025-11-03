# Troubleshooting Guide

## Common Issues

### Logs Not Appearing

**Problem**: No log output visible

**Solutions**:
1. Check if logger is enabled:
```rust
logger.enable();
```

2. Verify log level:
```rust
let mut config = LoggerConfig::default();
config.level = Level::Trace;  // Lower threshold
logger.configure(config);
```

3. Check global display settings:
```rust
config.global_console_display = true;
config.global_file_storage = true;
```

4. Verify sinks exist:
```rust
println!("Sinks: {}", logger.get_sink_count());
println!("IDs: {:?}", logger.list_sinks());
```

### File Not Created

**Problem**: Log file doesn't exist

**Solutions**:
1. Check path is correct
2. Verify parent directories exist (Logly creates them automatically)
3. Check file permissions
4. Verify `global_file_storage = true`

### Rotation Not Working

**Problem**: Files not rotating

**Solutions**:
1. Verify rotation configuration:
```rust
SinkConfig {
    rotation: Some("daily".to_string()),
    size_limit: Some(10 * 1024 * 1024),
    ..Default::default()
}
```

2. Check if size/time threshold is reached
3. Ensure retention policy is set

### GPU Not Available

**Problem**: GPU acceleration fails

**Solutions**:
1. Install CUDA toolkit
2. Compile with `--features gpu`
3. Check GPU is detected by system
4. Use CPU fallback (automatic)

### Performance Issues

**Problem**: Logging is slow

**Solutions**:
1. Enable async writing:
```rust
SinkConfig {
    async_write: true,
    buffer_size: 16384,
    ..Default::default()
}
```

2. Reduce log level in production:
```rust
config.level = Level::Info;  // Skip DEBUG/TRACE
```

3. Use file storage instead of console
4. Increase buffer sizes

### Configuration File Not Loaded

**Problem**: logly.toml settings ignored

**Solutions**:
1. Verify file is in project root
2. Check file name is exactly `logly.toml`
3. Validate TOML syntax
4. Check for duplicate config files
5. Ensure scanning is enabled (default)

### Memory Usage High

**Problem**: High memory consumption

**Solutions**:
1. Reduce buffer sizes:
```rust
config.gpu_buffer_size = 512 * 1024;  // 512KB
```

2. Disable GPU if not needed
3. Use smaller retention periods
4. Enable rotation with size limits

## Error Messages

### "Sink not found"
**Cause**: Invalid sink ID
**Solution**: Use valid ID from `add_sink()` return value

### "Invalid log level"
**Cause**: Unknown level name
**Solution**: Use: TRACE, DEBUG, INFO, SUCCESS, WARNING, ERROR, FAIL, CRITICAL

### "Failed to create file"
**Cause**: Permission denied or invalid path
**Solution**: Check permissions and path validity

### "CUDA device not available"
**Cause**: No GPU or CUDA not installed
**Solution**: Install CUDA or disable GPU feature

## Debug Mode

Enable debug mode to see internal operations:

```rust
let mut config = LoggerConfig::default();
config.debug_mode = true;
config.debug_log_file = Some(PathBuf::from("logly_debug.log"));
logger.configure(config);
```

Output shows:
- Sink operations
- Configuration changes
- GPU status
- Callback execution
- File operations

## Reporting Issues

If you encounter a bug, please report it:

**Rust crate**: https://github.com/muhammad-fiaz/logly-rs/issues
**Python package**: https://github.com/muhammad-fiaz/logly/issues

Include:
- Rust version
- Logly version
- Operating system
- Minimal reproduction code
- Error messages
- Debug log output

## Performance Profiling

Profile logging performance:

```rust
use std::time::Instant;

let start = Instant::now();
for i in 0..10000 {
    logger.info(format!("Message {}", i))?;
}
let duration = start.elapsed();
println!("10,000 logs in {:?}", duration);
```

## Best Practices

1. **Start simple**: Use defaults first
2. **Enable debug mode**: When troubleshooting
3. **Check examples**: Reference working code
4. **Read docs**: Check relevant guide
5. **Test incrementally**: Add features one at a time
