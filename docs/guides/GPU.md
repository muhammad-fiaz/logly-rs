# GPU Support Guide

## Overview

Logly provides optional GPU/CUDA acceleration for high-throughput logging scenarios.

## Installation

```toml
[dependencies]
logly = { version = "0.1.7", features = ["gpu"] }
```

**Requirements**:
- CUDA Toolkit installed
- NVIDIA GPU with CUDA support
- Compile with `--features gpu`

## Basic Usage

```rust
use logly::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let logger = Logger::new();
    
    let mut config = LoggerConfig::default();
    config.enable_gpu = true;
    config.gpu_buffer_size = 2 * 1024 * 1024;  // 2MB
    
    logger.configure(config);
    logger.enable_gpu()?;
    
    // Check GPU status
    println!("{}", logger.gpu_info());
    
    // Logs are now accelerated by GPU
    for i in 0..10000 {
        logger.info(format!("Message {}", i))?;
    }
    
    Ok(())
}
```

## Configuration

```rust
let mut config = LoggerConfig::default();
config.enable_gpu = true;
config.gpu_buffer_size = 2 * 1024 * 1024;  // 2MB buffer
logger.configure(config);
```

## GPU Operations

### Enable GPU
```rust
logger.enable_gpu()?;
```

### Disable GPU
```rust
logger.disable_gpu();
```

### Check Status
```rust
let info = logger.gpu_info();
println!("{}", info);
```

## Graceful Fallback

If GPU is unavailable:
- Logs warning message
- Falls back to CPU logging
- No errors or crashes
- Application continues normally

```rust
match logger.enable_gpu() {
    Ok(_) => println!("GPU enabled"),
    Err(e) => println!("GPU unavailable: {}, using CPU", e),
}
```

## Performance

GPU acceleration is beneficial for:
- High-throughput logging (>10,000 logs/sec)
- Real-time systems
- Data-intensive applications
- Parallel processing workloads

## Limitations

- Experimental feature
- Requires CUDA toolkit
- NVIDIA GPUs only
- Additional memory overhead
- Not needed for typical applications

## Example Output

```
GPU Logging: Enabled
Device: CUDA Device 0
Buffer Size: 2097152 bytes
Status: Active
```

## Troubleshooting

### GPU Not Available
```
Error: CUDA device not available
```
**Solution**: Install CUDA toolkit and ensure GPU is detected

### Compilation Error
```
Error: cudarc not found
```
**Solution**: Compile with `--features gpu`

### Performance Not Improved
**Solution**: GPU acceleration helps with high-volume logging. For typical applications, CPU logging is sufficient.

## Best Practices

1. **Use for high-throughput**: Only enable for >10,000 logs/sec
2. **Monitor memory**: GPU buffer uses device memory
3. **Test fallback**: Ensure application works without GPU
4. **Profile first**: Measure if GPU actually improves performance
