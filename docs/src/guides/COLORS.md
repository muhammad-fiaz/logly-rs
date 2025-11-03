# Colored Output

ANSI color support for terminal output.

## Enable Colors

```rust
let mut config = LoggerConfig::default();
config.color = true;
logger.configure(config);
```

## Default Colors

Built-in levels have default colors:
- TRACE: Cyan
- DEBUG: Blue
- INFO: Green
- SUCCESS: Bright Green
- WARNING: Yellow
- ERROR: Red
- FAIL: Bright Red
- CRITICAL: Bright Red + Bold

## Custom Color Callback

Override default colors:

```rust
logger.add_color_callback(|level, message| {
    match level {
        Level::Info => format!("\x1b[32m{}\x1b[0m", message),
        Level::Error => format!("\x1b[31;1m{}\x1b[0m", message),
        _ => message.to_string(),
    }
});
```

## ANSI Codes

Format: `\x1b[CODEm`

### Foreground Colors
- `30` - Black
- `31` - Red
- `32` - Green
- `33` - Yellow
- `34` - Blue
- `35` - Magenta
- `36` - Cyan
- `37` - White

### Bright Colors
- `90-97` - Bright variants

### Styles
- `1` - Bold
- `4` - Underline
- `0` - Reset

## Global Control

```rust
let mut config = LoggerConfig::default();
config.global_color_display = true;
logger.configure(config);
```

## See Also

- [Formatting](./FORMATTING.md)
- [Custom Levels](./CUSTOM_LEVELS.md)
