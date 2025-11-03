use logly::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let logger = Logger::new();

    // Add console sink with colors enabled (default)
    logger.add_sink(SinkConfig::default())?;

    println!("Testing default colors:");
    logger.trace("This is TRACE (Cyan)".to_string())?;
    logger.debug("This is DEBUG (Blue)".to_string())?;
    logger.info("This is INFO (White)".to_string())?;
    logger.success("This is SUCCESS (Green)".to_string())?;
    logger.warning("This is WARNING (Yellow)".to_string())?;
    logger.error("This is ERROR (Red)".to_string())?;
    logger.fail("This is FAIL (Magenta)".to_string())?;
    logger.critical("This is CRITICAL (Bright Red)".to_string())?;

    println!("\nTesting custom color callback:");
    logger.add_color_callback(|level, message| {
        // Custom bold formatting
        format!(
            "\x1b[1;{}m[{}]\x1b[0m {}",
            level.default_color(),
            level.as_str(),
            message
        )
    });

    logger.info("Custom colored INFO".to_string())?;
    logger.error("Custom colored ERROR".to_string())?;

    Ok(())
}
