use logly::prelude::*;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Logly Color Features Demo ===\n");
    
    // 1. Default Built-in Colors
    println!("1. Default Built-in Colors:");
    let logger = Logger::new();
    logger.add_sink(SinkConfig::default())?;
    
    logger.trace("TRACE - Cyan (36)".to_string())?;
    logger.debug("DEBUG - Blue (34)".to_string())?;
    logger.info("INFO - White (37)".to_string())?;
    logger.success("SUCCESS - Green (32)".to_string())?;
    logger.warning("WARNING - Yellow (33)".to_string())?;
    logger.error("ERROR - Red (31)".to_string())?;
    logger.fail("FAIL - Magenta (35)".to_string())?;
    logger.critical("CRITICAL - Bright Red (91)".to_string())?;
    
    // 2. Custom Colors Per Level
    println!("\n2. Custom Colors Per Level:");
    let logger2 = Logger::new();
    let mut config = LoggerConfig::default();
    
    // Customize colors for specific levels
    config.level_colors.insert(Level::Info, "92".to_string());      // Bright Green
    config.level_colors.insert(Level::Warning, "93".to_string());   // Bright Yellow
    config.level_colors.insert(Level::Error, "95".to_string());     // Bright Magenta
    
    logger2.configure(config);
    logger2.add_sink(SinkConfig::default())?;
    
    logger2.info("INFO with custom Bright Green".to_string())?;
    logger2.warning("WARNING with custom Bright Yellow".to_string())?;
    logger2.error("ERROR with custom Bright Magenta".to_string())?;
    
    // 3. Custom Log Levels with Custom Colors
    println!("\n3. Custom Log Levels with Custom Colors:");
    let logger3 = Logger::new();
    logger3.add_sink(SinkConfig::default())?;
    
    logger3.add_custom_level("NOTICE".to_string(), 35, "96".to_string())?;  // Bright Cyan
    logger3.add_custom_level("ALERT".to_string(), 42, "94".to_string())?;   // Bright Blue
    
    logger3.log_custom("NOTICE", "Custom NOTICE level - Bright Cyan".to_string())?;
    logger3.log_custom("ALERT", "Custom ALERT level - Bright Blue".to_string())?;
    
    // 4. Per-Sink Color Configuration
    println!("\n4. Per-Sink Color Configuration:");
    let logger4 = Logger::new();
    
    // Sink with colors
    let mut sink_colored = SinkConfig::default();
    sink_colored.color = true;
    logger4.add_sink(sink_colored)?;
    
    logger4.info("This sink has colors enabled".to_string())?;
    
    // 5. Disable Colors Globally
    println!("\n5. Disable Colors Globally:");
    let logger5 = Logger::new();
    let mut config5 = LoggerConfig::default();
    config5.global_color_display = false;
    logger5.configure(config5);
    logger5.add_sink(SinkConfig::default())?;
    
    logger5.info("No colors - global_color_display = false".to_string())?;
    logger5.error("No colors on error either".to_string())?;
    
    // 6. Color Callback (Custom Formatting)
    println!("\n6. Color Callback (Custom Formatting):");
    let logger6 = Logger::new();
    logger6.add_sink(SinkConfig::default())?;
    
    logger6.add_color_callback(|level, message| {
        // Custom bold + underline formatting
        let color = level.default_color();
        format!("\x1b[1;4;{}m[{}]\x1b[0m {}", color, level.as_str(), message)
    });
    
    logger6.info("Custom callback - Bold + Underline".to_string())?;
    logger6.error("Custom callback on ERROR".to_string())?;
    
    // 7. Level-Specific Color Control
    println!("\n7. Level-Specific Color Control:");
    let logger7 = Logger::new();
    let mut config7 = LoggerConfig::default();
    
    // Enable colors only for specific levels
    config7.color_levels.insert(Level::Error, true);
    config7.color_levels.insert(Level::Critical, true);
    config7.color_levels.insert(Level::Info, false);
    
    logger7.configure(config7);
    logger7.add_sink(SinkConfig::default())?;
    
    logger7.info("INFO - no color (disabled)".to_string())?;
    logger7.error("ERROR - with color (enabled)".to_string())?;
    logger7.critical("CRITICAL - with color (enabled)".to_string())?;
    
    // 8. Multiple Sinks with Different Color Settings
    println!("\n8. Multiple Sinks with Different Color Settings:");
    let logger8 = Logger::new();
    
    // Console sink with colors
    let mut console_config = SinkConfig::default();
    console_config.color = true;
    logger8.add_sink(console_config)?;
    
    logger8.info("Multiple sinks - colors enabled".to_string())?;
    logger8.success("All sinks receive colored output".to_string())?;
    
    // 9. Custom Format with Colors
    println!("\n9. Custom Format with Colors:");
    let logger9 = Logger::new();
    let mut format_config = SinkConfig::default();
    format_config.format = Some("{time:HH:mm:ss} [{level}] {message}".to_string());
    format_config.color = true;
    logger9.add_sink(format_config)?;
    
    logger9.info("Custom format with colors".to_string())?;
    logger9.warning("Colors work in custom formats too".to_string())?;
    
    // 10. Filter + Colors
    println!("\n10. Filter + Colors:");
    let logger10 = Logger::new();
    let mut filter_config = SinkConfig::default();
    filter_config.filter_min_level = Some(Level::Warning);
    filter_config.color = true;
    logger10.add_sink(filter_config)?;
    
    logger10.info("This won't show (below WARNING)".to_string())?;
    logger10.warning("This shows with color (WARNING+)".to_string())?;
    logger10.error("This shows with color (ERROR)".to_string())?;
    
    println!("\n=== Demo Complete ===");
    Ok(())
}
