# Version Checking

Automatic update notifications for new releases.

## Overview

Logly can automatically check for new versions on crates.io and notify you of updates.

## Enabled by Default

Version checking is enabled by default with the `auto-update-check` feature.

## Check for Updates

```rust
use logly::prelude::*;

let logger = Logger::new();

if let Ok(Some(msg)) = logger.check_version() {
    println!("{}", msg);
}
```

## Disable Version Checking

### At Compile Time

```toml
[dependencies]
logly = { version = "0.0.4", default-features = false, features = ["async", "rotation", "json", "colors"] }
```

### At Runtime

```rust
let mut config = LoggerConfig::default();
config.enable_version_check = false;
logger.configure(config);
```

## Example Output

```
New version available: 0.0.5
Current version: 0.0.4
Update with: cargo update logly
```

## See Also

- [Installation](../INSTALLATION.md)
- [Configuration](../CONFIGURATION.md)
