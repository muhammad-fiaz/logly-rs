// logly/tests/integration_tests.rs

use logly::logly::*;

#[test]
fn test_start_and_stop_logging() {
    let logger = Logger::new();

    assert!(logger.start_logging("test_log.txt").is_ok());

    logger.info("Key1", "Value1", LogColor::Cyan);
    logger.warn("Key2", "Value2", LogColor::Yellow);

    logger.stop_logging();

    // Assert that the log file was created and contains expected content
    let content = std::fs::read_to_string("test_log.txt").expect("Error reading log file");
    assert!(content.contains("Key1"));
    assert!(content.contains("Value1"));
    assert!(content.contains("Key2"));
    assert!(content.contains("Value2"));
}

