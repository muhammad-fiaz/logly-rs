# Introduction

Logly-rs is a high-performance, structured logging library for Rust designed to provide all the features of the Python logly library with native Rust performance and safety guarantees.

## Key Features

### Performance
- **Zero-copy logging**: Minimal allocations
- **Async I/O**: Non-blocking writes
- **Lock-free operations**: Optimized for concurrency
- **GPU/CPU support**: Hardware-accelerated operations (optional)

### Flexibility
- **8 log levels**: TRACE, DEBUG, INFO, SUCCESS, WARNING, ERROR, CRITICAL, FAIL
- **Multiple sinks**: Console, file, custom outputs
- **Custom formatting**: Template strings with placeholders
- **Colored output**: ANSI colors with custom callbacks

### Reliability
- **Thread-safe**: Safe concurrent logging
- **Error handling**: Comprehensive error types
- **File rotation**: Time and size-based rotation
- **Retention policies**: Automatic cleanup

## Design Philosophy

Logly-rs follows these principles:

1. **Performance First**: Optimized for high-throughput scenarios
2. **Type Safety**: Leverage Rust's type system
3. **Zero Cost Abstractions**: No runtime overhead
4. **Modular Design**: Use only what you need
5. **Python Compatibility**: Match Python logly API where possible

## Comparison with Python Logly

| Feature | Python Logly | Logly-rs |
|---------|-------------|----------|
| Performance | Fast (Rust backend) | Native Rust (faster) |
| Memory Safety | Runtime checks | Compile-time guarantees |
| Async Support | ✓ | ✓ |
| File Rotation | ✓ | ✓ |
| JSON Logging | ✓ | ✓ |
| Custom Colors | ✓ | ✓ |
| GPU Support | Planned | ✓ (optional) |

## Next Steps

- [Installation](./INSTALLATION.md): Get started with logly-rs
- [Quick Start](./QUICK_START.md): Your first logging program
- [Configuration](./CONFIGURATION.md): Configure your logger
