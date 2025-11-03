// Complete configuration example showing all features

use logly::prelude::*;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Logly Configuration Example ===\n");

    // Create logger - works out of box with auto-sink
    let logger = Logger::new();

    // Example 1: Default configuration (auto-sink enabled)
    println!("1. Default Configuration (auto-sink):");
    logger.info("This works out of the box!".to_string())?;

    // Example 2: Custom configuration
    println!("\n2. Custom Configuration:");
    let mut config = LoggerConfig::default();

    // Global display settings
    config.global_color_display = true; // Enable/disable colors globally
    config.global_console_display = true; // Enable/disable console output
    config.global_file_storage = true; // Enable/disable file storage

    // Log level
    config.level = Level::Trace;

    // Display options
    config.show_time = true;
    config.show_module = true;
    config.show_function = false;
    config.show_filename = false;
    config.show_lineno = false;

    // Features
    config.enable_callbacks = true;
    config.enable_exception_handling = true;
    config.enable_version_check = true;
    config.debug_mode = false;

    // Debug logging to file
    std::fs::create_dir_all("logs")?;
    config.debug_log_file = Some(PathBuf::from("logs/logly_debug.log"));

    // Auto-sink (enabled by default)
    config.auto_sink = true;

    logger.configure(config);

    logger.trace("Trace with custom config".to_string())?;
    logger.info("Info with custom config".to_string())?;

    // Example 3: Disable console, enable file storage only
    println!("\n3. File Storage Only (no console output):");
    let mut file_only_config = LoggerConfig::default();
    file_only_config.global_console_display = false; // No console
    file_only_config.global_file_storage = true; // Only files
    file_only_config.auto_sink = false; // Disable auto-sink

    let file_logger = Logger::new();
    file_logger.configure(file_only_config);

    // Add file sink manually
    std::fs::create_dir_all("logs")?;
    file_logger.add_sink(SinkConfig {
        path: Some(PathBuf::from("logs/file_only.log")),
        ..Default::default()
    })?;

    file_logger.info("This goes to file only, not console".to_string())?;
    println!("  (Check logs/file_only.log)");

    // Example 4: Console only, no file storage
    println!("\n4. Console Only (no file storage):");
    let mut console_only_config = LoggerConfig::default();
    console_only_config.global_console_display = true; // Console enabled
    console_only_config.global_file_storage = false; // No file storage

    let console_logger = Logger::new();
    console_logger.configure(console_only_config);
    console_logger.add_sink(SinkConfig::default())?;

    console_logger.info("This displays on console only".to_string())?;

    // Example 5: Disable both (no output)
    println!("\n5. No Output (both disabled):");
    let mut no_output_config = LoggerConfig::default();
    no_output_config.global_console_display = false;
    no_output_config.global_file_storage = false;

    let silent_logger = Logger::new();
    silent_logger.configure(no_output_config);
    silent_logger.add_sink(SinkConfig::default())?;

    silent_logger.info("This message is not displayed or stored".to_string())?;
    println!("  (No output - both console and storage disabled)");

    // Example 6: Debug mode with file logging
    println!("\n6. Debug Mode:");
    let mut debug_config = LoggerConfig::default();
    debug_config.debug_mode = true;
    debug_config.debug_log_file = Some(PathBuf::from("logs/logly_debug.log"));

    let debug_logger = Logger::new();
    debug_logger.configure(debug_config);
    debug_logger.add_sink(SinkConfig::default())?;

    debug_logger.info("Debug mode logs all logly operations".to_string())?;
    println!("  (Check logs/logly_debug.log for debug info)");

    // Example 7: File rotation with yearly option
    println!("\n7. File Rotation (including yearly):");
    let rotation_logger = Logger::new();

    rotation_logger.add_sink(SinkConfig {
        path: Some(PathBuf::from("logs/hourly.log")),
        rotation: Some("hourly".to_string()),
        retention: Some(24),
        ..Default::default()
    })?;

    rotation_logger.add_sink(SinkConfig {
        path: Some(PathBuf::from("logs/daily.log")),
        rotation: Some("daily".to_string()),
        retention: Some(7),
        ..Default::default()
    })?;

    rotation_logger.add_sink(SinkConfig {
        path: Some(PathBuf::from("logs/weekly.log")),
        rotation: Some("weekly".to_string()),
        retention: Some(4),
        ..Default::default()
    })?;

    rotation_logger.add_sink(SinkConfig {
        path: Some(PathBuf::from("logs/monthly.log")),
        rotation: Some("monthly".to_string()),
        retention: Some(12),
        ..Default::default()
    })?;

    rotation_logger.add_sink(SinkConfig {
        path: Some(PathBuf::from("logs/yearly.log")),
        rotation: Some("yearly".to_string()),
        retention: Some(5),
        ..Default::default()
    })?;

    rotation_logger.info("Logged to all rotation intervals".to_string())?;
    println!("  ✓ Hourly, Daily, Weekly, Monthly, Yearly rotation configured");

    // Example 8: Runtime configuration changes
    println!("\n8. Runtime Configuration Changes:");
    let runtime_logger = Logger::new();
    runtime_logger.add_sink(SinkConfig::default())?;

    runtime_logger.info("Initial message".to_string())?;

    // Change configuration at runtime
    let mut new_config = LoggerConfig::default();
    new_config.level = Level::Warning;
    runtime_logger.configure(new_config);

    runtime_logger.debug("This won't show (below WARNING)".to_string())?;
    runtime_logger.warning("This shows (WARNING level)".to_string())?;

    // Example 9: Enable/Disable features at runtime
    println!("\n9. Runtime Feature Control:");
    let feature_logger = Logger::new();
    feature_logger.add_sink(SinkConfig::default())?;

    feature_logger.info("Logger enabled".to_string())?;

    feature_logger.disable();
    feature_logger.info("This won't show (logger disabled)".to_string())?;

    feature_logger.enable();
    feature_logger.info("Logger re-enabled".to_string())?;

    // Example 10: Version checking
    println!("\n10. Version Checking:");
    if let Ok(Some(update_msg)) = logger.check_version() {
        println!("{}", update_msg);
    } else {
        println!(
            "  ✓ You're using the latest version: {}",
            logger.current_version()
        );
    }

    println!("\n=== Configuration Examples Complete ===");
    println!("\nKey Features:");
    println!("  ✓ Works out of box with auto-sink");
    println!("  ✓ Global console display control");
    println!("  ✓ Global file storage control");
    println!("  ✓ Debug mode with file logging");
    println!("  ✓ All rotation intervals (hourly to yearly)");
    println!("  ✓ Runtime configuration changes");
    println!("  ✓ Enable/disable at runtime");
    println!("  ✓ Auto-update checking (enabled by default)");

    Ok(())
}
