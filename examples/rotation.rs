// File rotation and retention example

use logly::prelude::*;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let logger = Logger::new();

    println!("=== File Rotation Example ===\n");

    // Size-based rotation
    let size_config = SinkConfig {
        path: Some(PathBuf::from("logs/size_rotation.log")),
        size_limit: Some(1024 * 100), // 100KB
        retention: Some(5),           // Keep 5 files
        ..Default::default()
    };
    logger.add_sink(size_config)?;
    println!("✓ Added sink with size-based rotation (100KB, keep 5 files)");

    // Time-based rotation
    let time_config = SinkConfig {
        path: Some(PathBuf::from("logs/time_rotation.log")),
        rotation: Some("hourly".to_string()),
        retention: Some(24), // Keep 24 hours
        ..Default::default()
    };
    logger.add_sink(time_config)?;
    println!("✓ Added sink with time-based rotation (hourly, keep 24 files)");

    // Combined rotation
    let combined_config = SinkConfig {
        path: Some(PathBuf::from("logs/combined_rotation.log")),
        rotation: Some("daily".to_string()),
        size_limit: Some(10 * 1024 * 1024), // 10MB
        retention: Some(7),                 // Keep 7 days
        ..Default::default()
    };
    logger.add_sink(combined_config)?;
    println!("✓ Added sink with combined rotation (daily OR 10MB, keep 7 files)");

    println!("\nGenerating log messages...\n");

    // Generate logs
    for i in 0..1000 {
        logger.info(format!(
            "Log message #{} - Testing rotation functionality",
            i
        ))?;

        if i % 100 == 0 {
            logger.success(format!("Checkpoint: {} messages logged", i))?;
        }
    }

    println!("\n✓ Generated 1000 log messages");
    println!("✓ Check logs/ directory for rotated files");

    println!("\n=== Example Complete ===");

    Ok(())
}
