// Integration tests for logly

use logly::prelude::*;
use tempfile::TempDir;

#[test]
fn test_basic_logging() {
    let logger = Logger::new();
    let result = logger.add_sink(SinkConfig::default());
    assert!(result.is_ok());

    assert!(logger.info("Test info message".to_string()).is_ok());
    assert!(logger.debug("Test debug message".to_string()).is_ok());
    assert!(logger.error("Test error message".to_string()).is_ok());
}

#[test]
fn test_all_log_levels() {
    let logger = Logger::new();
    logger.add_sink(SinkConfig::default()).unwrap();

    assert!(logger.trace("Trace message".to_string()).is_ok());
    assert!(logger.debug("Debug message".to_string()).is_ok());
    assert!(logger.info("Info message".to_string()).is_ok());
    assert!(logger.success("Success message".to_string()).is_ok());
    assert!(logger.warning("Warning message".to_string()).is_ok());
    assert!(logger.error("Error message".to_string()).is_ok());
    assert!(logger.fail("Fail message".to_string()).is_ok());
    assert!(logger.critical("Critical message".to_string()).is_ok());
}

#[test]
fn test_level_priority() {
    assert!(Level::Trace < Level::Debug);
    assert!(Level::Debug < Level::Info);
    assert!(Level::Info < Level::Success);
    assert!(Level::Success < Level::Warning);
    assert!(Level::Warning < Level::Error);
    assert!(Level::Error < Level::Fail);
    assert!(Level::Fail < Level::Critical);

    assert_eq!(Level::Trace.priority(), 5);
    assert_eq!(Level::Debug.priority(), 10);
    assert_eq!(Level::Info.priority(), 20);
    assert_eq!(Level::Success.priority(), 25);
    assert_eq!(Level::Warning.priority(), 30);
    assert_eq!(Level::Error.priority(), 40);
    assert_eq!(Level::Fail.priority(), 45);
    assert_eq!(Level::Critical.priority(), 50);
}

#[test]
fn test_file_logging() {
    let temp_dir = TempDir::new().unwrap();
    let log_path = temp_dir.path().join("test.log");

    let logger = Logger::new();
    let config = SinkConfig {
        path: Some(log_path.clone()),
        ..Default::default()
    };
    logger.add_sink(config).unwrap();

    logger.info("File test message".to_string()).unwrap();
    
    std::thread::sleep(std::time::Duration::from_millis(100));

    assert!(log_path.exists());
}

#[test]
fn test_sink_management() {
    let logger = Logger::new();
    
    let id1 = logger.add_sink(SinkConfig::default()).unwrap();
    let _id2 = logger.add_sink(SinkConfig::default()).unwrap();
    
    assert_eq!(logger.get_sink_count(), 2);
    
    assert!(logger.remove_sink(id1));
    assert_eq!(logger.get_sink_count(), 1);
    
    let removed = logger.remove_all_sinks();
    assert_eq!(removed, 1);
    assert_eq!(logger.get_sink_count(), 0);
}

#[test]
fn test_enable_disable() {
    let logger = Logger::new();
    logger.add_sink(SinkConfig::default()).unwrap();

    logger.disable();
    assert!(logger.info("Should not log".to_string()).is_ok());

    logger.enable();
    assert!(logger.info("Should log".to_string()).is_ok());
}

#[test]
fn test_context_binding() {
    let logger = Logger::new();
    logger.add_sink(SinkConfig::default()).unwrap();

    logger.bind("user_id".to_string(), serde_json::json!("12345"));
    logger.bind("session".to_string(), serde_json::json!("abc-def"));

    assert!(logger.info("Message with context".to_string()).is_ok());

    logger.unbind("user_id");
    logger.clear_bindings();
}

#[test]
fn test_custom_levels() {
    let logger = Logger::new();
    
    let result = logger.add_custom_level("NOTICE".to_string(), 35, "95".to_string());
    assert!(result.is_ok());

    let duplicate = logger.add_custom_level("NOTICE".to_string(), 35, "95".to_string());
    assert!(duplicate.is_err());

    assert!(logger.remove_custom_level("NOTICE"));
    assert!(!logger.remove_custom_level("NONEXISTENT"));
}

#[test]
fn test_callbacks() {
    let logger = Logger::new();
    logger.add_sink(SinkConfig::default()).unwrap();

    let callback_executed = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
    let callback_flag = callback_executed.clone();

    logger.add_log_callback(move |_record| {
        callback_flag.store(true, std::sync::atomic::Ordering::SeqCst);
        Ok(())
    });

    logger.info("Test callback".to_string()).unwrap();
    std::thread::sleep(std::time::Duration::from_millis(50));

    assert!(callback_executed.load(std::sync::atomic::Ordering::SeqCst));
}

#[test]
fn test_version_info() {
    let logger = Logger::new();
    let version = logger.current_version();
    assert_eq!(version, "0.0.4");
}

#[test]
fn test_debug_mode() {
    let logger = Logger::new();
    logger.enable_debug();
    
    assert!(logger.info("Debug mode test".to_string()).is_ok());
    
    logger.disable_debug();
}

#[test]
fn test_level_from_string() {
    use std::str::FromStr;
    
    assert_eq!(Level::from_str("TRACE").unwrap(), Level::Trace);
    assert_eq!(Level::from_str("debug").unwrap(), Level::Debug);
    assert_eq!(Level::from_str("INFO").unwrap(), Level::Info);
    assert_eq!(Level::from_str("success").unwrap(), Level::Success);
    assert_eq!(Level::from_str("WARNING").unwrap(), Level::Warning);
    assert_eq!(Level::from_str("error").unwrap(), Level::Error);
    assert_eq!(Level::from_str("FAIL").unwrap(), Level::Fail);
    assert_eq!(Level::from_str("critical").unwrap(), Level::Critical);
    
    assert!(Level::from_str("INVALID").is_err());
}

#[test]
fn test_concurrent_logging() {
    use std::sync::Arc;
    use std::thread;

    let logger = Arc::new(Logger::new());
    logger.add_sink(SinkConfig::default()).unwrap();

    let mut handles = vec![];

    for i in 0..10 {
        let logger_clone = Arc::clone(&logger);
        let handle = thread::spawn(move || {
            for j in 0..100 {
                logger_clone.info(format!("Thread {} - Message {}", i, j)).unwrap();
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
