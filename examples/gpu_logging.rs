// GPU logging example (requires --features gpu)

use logly::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let logger = Logger::new();
    
    let mut config = LoggerConfig::default();
    config.enable_gpu = true;
    config.gpu_buffer_size = 2 * 1024 * 1024; // 2MB
    config.debug_mode = true;
    
    logger.configure(config);
    
    println!("=== GPU Logging Example ===\n");
    
    // Display GPU info
    println!("{}\n", logger.gpu_info());
    
    // Try to enable GPU
    match logger.enable_gpu() {
        Ok(_) => {
            println!("GPU logging enabled successfully!");
            
            logger.add_sink(SinkConfig::default())?;
            
            // Log messages that will be processed on GPU
            for i in 0..100 {
                logger.info(format!("GPU log message #{}", i))?;
            }
            
            println!("\nLogged 100 messages with GPU acceleration");
        }
        Err(e) => {
            println!("GPU logging not available: {}", e);
            println!("\nTo enable GPU support:");
            println!("  1. Install CUDA toolkit");
            println!("  2. Compile with: cargo run --example gpu_logging --features gpu");
        }
    }
    
    println!("\n=== Example Complete ===");
    
    Ok(())
}
