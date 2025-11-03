// Basic usage example

use logly::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create logger
    let logger = Logger::new();

    // Configure
    let config = LoggerConfig::default();
    logger.configure(config);

    // Add console sink
    let sink_config = SinkConfig {
        path: None,
        ..Default::default()
    };
    logger.add_sink(sink_config)?;

    // Log messages
    logger.info("Application started".to_string())?;
    logger.debug("Debug information".to_string())?;
    logger.success("Operation completed".to_string())?;
    logger.warning("Warning message".to_string())?;
    logger.error("Error occurred".to_string())?;

    Ok(())
}
