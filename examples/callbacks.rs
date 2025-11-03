// Callbacks and custom colors example

use logly::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let logger = Logger::new();
    
    let mut config = LoggerConfig::default();
    config.enable_callbacks = true;
    logger.configure(config);
    
    logger.add_sink(SinkConfig::default())?;
    
    println!("=== Callbacks Example ===\n");
    
    // Add log callback for monitoring
    logger.add_log_callback(|record| {
        if record.level >= Level::Error {
            println!("[MONITOR] High severity log detected: {}", record.message);
        }
        Ok(())
    });
    
    // Add color callback for custom formatting
    logger.add_color_callback(|level, message| {
        let color_code = match level {
            Level::Trace => "96",
            Level::Debug => "94",
            Level::Info => "92",
            Level::Success => "32;1",
            Level::Warning => "93",
            Level::Error => "91",
            Level::Fail => "95",
            Level::Critical => "91;1",
        };
        format!("\x1b[{}m● {}\x1b[0m", color_code, message)
    });
    
    // Add exception callback
    logger.add_exception_callback(|error, backtrace| {
        eprintln!("\n⚠️  EXCEPTION CAUGHT ⚠️");
        eprintln!("Error: {}", error);
        eprintln!("Backtrace:\n{}", backtrace);
    });
    
    // Test logging with callbacks
    logger.trace("Trace with custom color".to_string())?;
    logger.debug("Debug with custom color".to_string())?;
    logger.info("Info with custom color".to_string())?;
    logger.success("Success with custom color".to_string())?;
    logger.warning("Warning with custom color".to_string())?;
    logger.error("Error triggering monitor callback".to_string())?;
    logger.critical("Critical triggering monitor callback".to_string())?;
    
    println!("\n=== Example Complete ===");
    
    Ok(())
}
