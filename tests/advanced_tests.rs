// Advanced integration tests for logly

use logly::prelude::*;
use tempfile::TempDir;

#[test]
fn test_file_rotation_daily() {
    let temp_dir = TempDir::new().unwrap();
    let log_path = temp_dir.path().join("daily.log");

    let logger = Logger::new();
    let config = SinkConfig {
        path: Some(log_path.clone()),
        rotation: Some("daily".to_string()),
        ..Default::default()
    };
    logger.add_sink(config).unwrap();

    logger.info("Daily rotation test".to_string()).unwrap();
    std::thread::sleep(std::time::Duration::from_millis(100));

    assert!(log_path.exists());
}

#[test]
fn test_file_rotation_size() {
    let temp_dir = TempDir::new().unwrap();
    let log_path = temp_dir.path().join("size.log");

    let logger = Logger::new();
    let config = SinkConfig {
        path: Some(log_path.clone()),
        size_limit: Some(1024), // 1KB
        ..Default::default()
    };
    logger.add_sink(config).unwrap();

    for i in 0..100 {
        logger.info(format!("Size rotation test message {}", i)).unwrap();
    }
    
    std::thread::sleep(std::time::Duration::from_millis(200));
}

#[test]
fn test_json_format() {
    let temp_dir = TempDir::new().unwrap();
    let log_path = temp_dir.path().join("json.log");

    let logger = Logger::new();
    let mut config = LoggerConfig::default();
    config.json = true;
    logger.configure(config);

    let sink_config = SinkConfig {
        path: Some(log_path.clone()),
        ..Default::default()
    };
    logger.add_sink(sink_config).unwrap();

    logger.info("JSON format test".to_string()).unwrap();
    std::thread::sleep(std::time::Duration::from_millis(100));

    assert!(log_path.exists());
}

#[test]
fn test_level_filtering() {
    let logger = Logger::new();
    let mut config = LoggerConfig::default();
    config.level = Level::Warning;
    logger.configure(config);

    logger.add_sink(SinkConfig::default()).unwrap();

    // These should not log
    assert!(logger.trace("Should not log".to_string()).is_ok());
    assert!(logger.debug("Should not log".to_string()).is_ok());
    assert!(logger.info("Should not log".to_string()).is_ok());

    // These should log
    assert!(logger.warning("Should log".to_string()).is_ok());
    assert!(logger.error("Should log".to_string()).is_ok());
    assert!(logger.critical("Should log".to_string()).is_ok());
}

#[test]
fn test_multiple_sinks() {
    let temp_dir = TempDir::new().unwrap();
    let log_path1 = temp_dir.path().join("sink1.log");
    let log_path2 = temp_dir.path().join("sink2.log");

    let logger = Logger::new();
    
    logger.add_sink(SinkConfig {
        path: Some(log_path1.clone()),
        ..Default::default()
    }).unwrap();
    
    logger.add_sink(SinkConfig {
        path: Some(log_path2.clone()),
        ..Default::default()
    }).unwrap();

    logger.info("Multiple sinks test".to_string()).unwrap();
    std::thread::sleep(std::time::Duration::from_millis(100));

    assert!(log_path1.exists());
    assert!(log_path2.exists());
}

#[test]
fn test_async_logging() {
    let temp_dir = TempDir::new().unwrap();
    let log_path = temp_dir.path().join("async.log");

    let logger = Logger::new();
    let config = SinkConfig {
        path: Some(log_path.clone()),
        async_write: true,
        ..Default::default()
    };
    logger.add_sink(config).unwrap();

    for i in 0..1000 {
        logger.info(format!("Async message {}", i)).unwrap();
    }

    std::thread::sleep(std::time::Duration::from_millis(500));
    assert!(log_path.exists());
}

#[test]
fn test_color_callback() {
    let logger = Logger::new();
    logger.add_sink(SinkConfig::default()).unwrap();

    let callback_executed = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
    let callback_flag = callback_executed.clone();

    logger.add_color_callback(move |_level, message| {
        callback_flag.store(true, std::sync::atomic::Ordering::SeqCst);
        format!("[CUSTOM] {}", message)
    });

    logger.info("Color callback test".to_string()).unwrap();
    std::thread::sleep(std::time::Duration::from_millis(50));
}

#[test]
fn test_exception_callback() {
    let logger = Logger::new();
    let mut config = LoggerConfig::default();
    config.enable_exception_handling = true;
    logger.configure(config);

    let callback_executed = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
    let callback_flag = callback_executed.clone();

    logger.add_exception_callback(move |_error, _backtrace| {
        callback_flag.store(true, std::sync::atomic::Ordering::SeqCst);
    });

    logger.add_sink(SinkConfig::default()).unwrap();
    logger.info("Exception callback test".to_string()).unwrap();
}

#[test]
fn test_custom_level_priority() {
    let logger = Logger::new();
    
    logger.add_custom_level("NOTICE".to_string(), 35, "96".to_string()).unwrap();
    logger.add_custom_level("ALERT".to_string(), 42, "91".to_string()).unwrap();
    
    logger.add_sink(SinkConfig::default()).unwrap();
    
    assert!(logger.log_custom("NOTICE", "Notice message".to_string()).is_ok());
    assert!(logger.log_custom("ALERT", "Alert message".to_string()).is_ok());
    assert!(logger.log_custom("INVALID", "Invalid message".to_string()).is_err());
}

#[test]
fn test_global_console_display() {
    let logger = Logger::new();
    let mut config = LoggerConfig::default();
    config.global_console_display = false;
    config.global_file_storage = false;
    logger.configure(config);

    logger.add_sink(SinkConfig::default()).unwrap();
    
    // Should not output anywhere
    assert!(logger.info("Silent mode test".to_string()).is_ok());
}

#[test]
fn test_global_file_storage() {
    let temp_dir = TempDir::new().unwrap();
    let log_path = temp_dir.path().join("storage.log");

    let logger = Logger::new();
    let mut config = LoggerConfig::default();
    config.global_file_storage = false;
    config.global_console_display = true; // Keep console enabled
    logger.configure(config);

    logger.add_sink(SinkConfig {
        path: Some(log_path.clone()),
        ..Default::default()
    }).unwrap();

    logger.info("File storage disabled".to_string()).unwrap();
    std::thread::sleep(std::time::Duration::from_millis(200));

    // File might be created but should be empty or minimal
    // The global_file_storage flag prevents writing to file sinks
    // This test verifies the flag works without causing errors
    assert!(logger.info("Another message".to_string()).is_ok());
}

#[test]
fn test_retention_policy() {
    let temp_dir = TempDir::new().unwrap();
    let log_path = temp_dir.path().join("retention.log");

    let logger = Logger::new();
    let config = SinkConfig {
        path: Some(log_path.clone()),
        retention: Some(3),
        ..Default::default()
    };
    logger.add_sink(config).unwrap();

    logger.info("Retention policy test".to_string()).unwrap();
    std::thread::sleep(std::time::Duration::from_millis(100));
}

#[test]
fn test_directory_creation() {
    let temp_dir = TempDir::new().unwrap();
    let log_path = temp_dir.path().join("nested/dir/test.log");

    let logger = Logger::new();
    let config = SinkConfig {
        path: Some(log_path.clone()),
        ..Default::default()
    };
    logger.add_sink(config).unwrap();

    logger.info("Directory creation test".to_string()).unwrap();
    std::thread::sleep(std::time::Duration::from_millis(100));

    assert!(log_path.exists());
    assert!(log_path.parent().unwrap().exists());
}

#[test]
fn test_custom_format() {
    let temp_dir = TempDir::new().unwrap();
    let log_path = temp_dir.path().join("format.log");

    let logger = Logger::new();
    let config = SinkConfig {
        path: Some(log_path.clone()),
        format: Some("{time:YYYY-MM-DD HH:mm:ss} [{level}] {message}".to_string()),
        ..Default::default()
    };
    logger.add_sink(config).unwrap();

    logger.info("Custom format test".to_string()).unwrap();
    std::thread::sleep(std::time::Duration::from_millis(100));

    assert!(log_path.exists());
}

#[test]
fn test_bound_fields_persistence() {
    let logger = Logger::new();
    logger.add_sink(SinkConfig::default()).unwrap();

    logger.bind("request_id".to_string(), serde_json::json!("req-123"));
    logger.bind("user".to_string(), serde_json::json!("alice"));

    logger.info("First message".to_string()).unwrap();
    logger.info("Second message".to_string()).unwrap();

    logger.unbind("request_id");
    logger.info("Third message".to_string()).unwrap();

    logger.clear_bindings();
    logger.info("Fourth message".to_string()).unwrap();
}

#[test]
fn test_high_throughput() {
    let logger = Logger::new();
    logger.add_sink(SinkConfig {
        async_write: true,
        ..Default::default()
    }).unwrap();

    let start = std::time::Instant::now();
    for i in 0..10000 {
        logger.info(format!("High throughput message {}", i)).unwrap();
    }
    let duration = start.elapsed();

    println!("Logged 10000 messages in {:?}", duration);
    assert!(duration.as_secs() < 5);
}

#[test]
fn test_sink_list() {
    let logger = Logger::new();
    
    let id1 = logger.add_sink(SinkConfig::default()).unwrap();
    let id2 = logger.add_sink(SinkConfig::default()).unwrap();
    let id3 = logger.add_sink(SinkConfig::default()).unwrap();

    let sinks = logger.list_sinks();
    assert_eq!(sinks.len(), 3);
    assert!(sinks.contains(&id1));
    assert!(sinks.contains(&id2));
    assert!(sinks.contains(&id3));
}

#[test]
fn test_level_from_priority() {
    assert_eq!(Level::from_priority(5), Some(Level::Trace));
    assert_eq!(Level::from_priority(10), Some(Level::Debug));
    assert_eq!(Level::from_priority(20), Some(Level::Info));
    assert_eq!(Level::from_priority(25), Some(Level::Success));
    assert_eq!(Level::from_priority(30), Some(Level::Warning));
    assert_eq!(Level::from_priority(40), Some(Level::Error));
    assert_eq!(Level::from_priority(45), Some(Level::Fail));
    assert_eq!(Level::from_priority(50), Some(Level::Critical));
    assert_eq!(Level::from_priority(99), None);
}

#[test]
fn test_runtime_config_changes() {
    let logger = Logger::new();
    logger.add_sink(SinkConfig::default()).unwrap();

    // Change level at runtime
    let mut config = LoggerConfig::default();
    config.level = Level::Error;
    logger.configure(config);

    logger.info("Should not log".to_string()).unwrap();
    logger.error("Should log".to_string()).unwrap();

    // Change back
    let mut config2 = LoggerConfig::default();
    config2.level = Level::Trace;
    logger.configure(config2);

    logger.trace("Should log now".to_string()).unwrap();
}
