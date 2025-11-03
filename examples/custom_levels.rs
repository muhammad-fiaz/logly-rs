// Custom log levels example

use logly::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let logger = Logger::new();
    logger.add_sink(SinkConfig::default())?;

    println!("=== Custom Log Levels Example ===\n");

    // Add custom levels
    logger.add_custom_level("NOTICE".to_string(), 35, "96".to_string())?;
    println!("✓ Added NOTICE level (priority: 35)");

    logger.add_custom_level("AUDIT".to_string(), 28, "93".to_string())?;
    println!("✓ Added AUDIT level (priority: 28)");

    logger.add_custom_level("SECURITY".to_string(), 48, "91;1".to_string())?;
    println!("✓ Added SECURITY level (priority: 48)");

    println!("\nStandard levels:");
    for level in Level::all_levels() {
        println!("  {} - priority: {}", level.as_str(), level.priority());
    }

    println!("\nLogging with custom levels:\n");

    // Standard levels
    logger.info("Standard INFO message".to_string())?;
    logger.warning("Standard WARNING message".to_string())?;
    logger.error("Standard ERROR message".to_string())?;

    // Custom levels
    logger.log_custom("AUDIT", "User login successful".to_string())?;
    logger.log_custom("NOTICE", "System maintenance scheduled".to_string())?;
    logger.log_custom(
        "SECURITY",
        "Unauthorized access attempt detected".to_string(),
    )?;

    // Try to add duplicate
    match logger.add_custom_level("NOTICE".to_string(), 35, "96".to_string()) {
        Ok(_) => println!("\nUnexpected: Duplicate level added"),
        Err(e) => println!("\n✓ Correctly rejected duplicate level: {}", e),
    }

    // Remove custom level
    if logger.remove_custom_level("AUDIT") {
        println!("✓ Removed AUDIT level");
    }

    println!("\n=== Example Complete ===");

    Ok(())
}
