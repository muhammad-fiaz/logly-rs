# Installation Guide

Complete installation guide for logly-rs.

## Table of Contents
- [Requirements](#requirements)
- [Basic Installation](#basic-installation)
- [Feature Flags](#feature-flags)
- [GPU Support](#gpu-support)
- [Verification](#verification)
- [Troubleshooting](#troubleshooting)

## Requirements

### Minimum Requirements
- **Rust**: 1.70.0 or later
- **Cargo**: Latest stable version
- **Operating System**: Windows, Linux, macOS

### Optional Requirements (for GPU support)
- **CUDA Toolkit**: 11.0 or later
- **NVIDIA GPU**: Compute Capability 3.5+
- **CUDA Driver**: Compatible with toolkit version

## Basic Installation

### Method 1: Add to Cargo.toml (Recommended)

Add logly to your `Cargo.toml`:

```toml
[dependencies]
logly = "0.0.4"
```

Then run:
```bash
cargo build
```

### Method 2: Cargo Add Command

```bash
cargo add logly
```

### Method 3: From Git Repository

```toml
[dependencies]
logly = { git = "https://github.com/muhammad-fiaz/logly-rs.git", tag = "v0.0.4" }
```

## Feature Flags

Logly supports several feature flags for customization:

### Default Features (Enabled Automatically)

```toml
[dependencies]
logly = "0.0.4"
# Includes: async, rotation, json, colors, auto-update-check
```

### Minimal Installation (No Default Features)

```toml
[dependencies]
logly = { version = "0.0.4", default-features = false }
```

### Custom Feature Selection

```toml
[dependencies]
logly = { version = "0.0.4", default-features = false, features = ["async", "colors"] }
```

### Available Features

| Feature | Description | Default |
|---------|-------------|---------|
| `async` | Async logging support | ✓ |
| `rotation` | File rotation support | ✓ |
| `json` | JSON format support | ✓ |
| `colors` | ANSI color support | ✓ |
| `auto-update-check` | Version checking | ✓ |
| `gpu` | GPU/CUDA acceleration | ✗ |

## GPU Support

### Prerequisites

1. **Install CUDA Toolkit**

**Linux:**
```bash
# Ubuntu/Debian
wget https://developer.download.nvidia.com/compute/cuda/repos/ubuntu2204/x86_64/cuda-keyring_1.0-1_all.deb
sudo dpkg -i cuda-keyring_1.0-1_all.deb
sudo apt-get update
sudo apt-get install cuda

# Verify installation
nvcc --version
```

**Windows:**
- Download CUDA Toolkit from [NVIDIA website](https://developer.nvidia.com/cuda-downloads)
- Run installer and follow instructions
- Add CUDA to PATH

**macOS:**
- CUDA is not officially supported on macOS
- Use CPU-only version

2. **Verify GPU**

```bash
nvidia-smi
```

### Install with GPU Support

```toml
[dependencies]
logly = { version = "0.0.4", features = ["gpu"] }
```

Or via command line:
```bash
cargo add logly --features gpu
```

### Build with GPU

```bash
cargo build --features gpu
```

## Verification

### Test Installation

Create `src/main.rs`:

```rust
use logly::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let logger = Logger::new();
    logger.add_sink(SinkConfig::default())?;
    
    logger.info("Logly installed successfully!".to_string())?;
    logger.success("All systems operational!".to_string())?;
    
    Ok(())
}
```

Run:
```bash
cargo run
```

Expected output:
```
[INFO] Logly installed successfully!
[SUCCESS] All systems operational!
```

### Test GPU Support (if enabled)

```rust
use logly::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let logger = Logger::new();
    logger.add_sink(SinkConfig::default())?;
    
    logger.enable_gpu()?;
    println!("{}", logger.gpu_info());
    
    logger.info("GPU logging enabled!".to_string())?;
    
    Ok(())
}
```

## Troubleshooting

### Common Issues

#### 1. Compilation Errors

**Error**: `failed to resolve: use of undeclared crate or module`

**Solution**:
```bash
cargo clean
cargo build
```

#### 2. GPU Feature Not Working

**Error**: `GPU logging not available`

**Solution**:
- Verify CUDA installation: `nvcc --version`
- Check GPU: `nvidia-smi`
- Rebuild with GPU feature: `cargo build --features gpu`

#### 3. Version Conflicts

**Error**: `failed to select a version for logly`

**Solution**:
```bash
cargo update
cargo build
```

#### 4. Missing Dependencies

**Error**: `could not find Cargo.toml`

**Solution**:
```bash
cargo init
# Then add logly to Cargo.toml
```

### Platform-Specific Issues

#### Windows

**Issue**: CUDA not found

**Solution**:
```cmd
set PATH=%PATH%;C:\Program Files\NVIDIA GPU Computing Toolkit\CUDA\v11.0\bin
```

#### Linux

**Issue**: Permission denied for log files

**Solution**:
```bash
sudo chmod 755 logs/
```

#### macOS

**Issue**: GPU feature not available

**Solution**: Use CPU-only version (GPU not supported on macOS)

### Getting Help

If you encounter issues:

1. **Check Documentation**: https://muhammad-fiaz.github.io/logly-rs
2. **Search Issues**: https://github.com/muhammad-fiaz/logly-rs/issues
3. **Create Issue**: https://github.com/muhammad-fiaz/logly-rs/issues/new
4. **Discord/Community**: Check repository for community links

### Debug Mode

Enable debug mode to troubleshoot:

```rust
let mut config = LoggerConfig::default();
config.debug_mode = true;
config.debug_log_file = Some(PathBuf::from("logly_debug.log"));
logger.configure(config);
```

## Next Steps

- Read [Quick Start Guide](QUICK_START.md)
- Explore [Examples](examples/)
- Check [Configuration Guide](CONFIGURATION.md)
- Review [API Documentation](https://docs.rs/logly)

## Version Information

Current version: **0.0.4**

Check for updates:
```rust
if let Ok(Some(msg)) = logger.check_version() {
    println!("{}", msg);
}
```

## License

MIT License - see [LICENSE](LICENSE) file for details.
