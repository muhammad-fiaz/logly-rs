// Advanced usage example with all features

use logly::prelude::*;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create logger with custom configuration
    let logger = Logger::new();
    
    let mut config = LoggerConfig::default();
    config.level = Level::Trace;
    config.color = true;
    config.enable_gpu = false; // Enable with --features gpu
    config.enable_callbacks = true;
    config.enable_exception_handling = true;
    config.enable_version_check = true;
    config.debug_mode = true;
    
    logger.configure(config);
    
    println!("=== Logly Advanced Example ===\n");
    println!("Version: {}\n", logger.current_version());
    
    // Add console sink
    logger.add_sink(SinkConfig::default())?;
    
    // Add file sink with rotation
    let file_config = SinkConfig {
        path: Some(PathBuf::from("logs/app.log")),
        rotation: Some("daily".to_string()),
        size_limit: Some(10 * 1024 * 1024), // 10MB
        retention: Some(7), // Keep 7 files
        async_write: true,
        buffer_size: 8192,
        ..Default::default()
    };
    logger.add_sink(file_config)?;
    
    // Add custom log level
    logger.add_custom_level("NOTICE".to_string(), 35, "96".to_string())?;
    
    // Add log callback
    logger.add_log_callback(|record| {
        println!("[CALLBACK] Logged: {} - {}", record.level, record.message);
        Ok(())
    });
    
    // Add color callback
    logger.add_color_callback(|level, message| {
        format!("\x1b[1m[{}]\x1b[0m {}", level.as_str(), message)
    });
    
    // Add exception callback
    logger.add_exception_callback(|error, backtrace| {
        eprintln!("Exception caught: {}\n{}", error, backtrace);
    });
    
    // Bind context
    logger.bind("app".to_string(), serde_json::json!("logly-demo"));
    logger.bind("version".to_string(), serde_json::json!("0.1.7"));
    
    // Log at all levels
    logger.trace("This is a trace message".to_string())?;
    logger.debug("This is a debug message".to_string())?;
    logger.info("This is an info message".to_string())?;
    logger.success("Operation completed successfully!".to_string())?;
    logger.warning("This is a warning message".to_string())?;
    logger.error("This is an error message".to_string())?;
    logger.fail("This operation failed".to_string())?;
    logger.critical("This is a critical message!".to_string())?;
    
    // Log custom level
    logger.log_custom("NOTICE", "This is a custom notice level".to_string())?;
    
    // GPU info
    println!("\n{}", logger.gpu_info());
    
    // Sink management
    println!("\nActive sinks: {}", logger.get_sink_count());
    println!("Sink IDs: {:?}", logger.list_sinks());
    
    // Check for updates
    if let Ok(Some(update_msg)) = logger.check_version() {
        println!("{}", update_msg);
    }
    
    println!("\n=== Example Complete ===");
    
    Ok(())
}
