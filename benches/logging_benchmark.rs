// Performance benchmarks for logly

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use logly::prelude::*;
use serde_json;
use std::fs;
use tempfile::TempDir;

fn export_bench_result(bench_path: &str) {
    let json_path = format!("target/criterion/{}/estimates.json", bench_path);
    if let Ok(content) = fs::read_to_string(&json_path) {
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) {
            let mean_point = json["mean"]["point_estimate"].as_f64().unwrap_or(0.0);
            let mean_std_err = json["mean"]["standard_error"].as_f64().unwrap_or(0.0);
            let median_point = json["median"]["point_estimate"].as_f64().unwrap_or(0.0);
            let median_std_err = json["median"]["standard_error"].as_f64().unwrap_or(0.0);
            let summary = format!(
                "Benchmark: {}\nMean: {:.2} ns ± {:.2} ns\nMedian: {:.2} ns ± {:.2} ns\n",
                bench_path, mean_point, mean_std_err, median_point, median_std_err
            );
            let txt_path = format!("target/criterion/{}/summary.txt", bench_path);
            fs::write(&txt_path, summary).ok();
        }
    }
}

fn bench_basic_logging(c: &mut Criterion) {
    let logger = Logger::new();
    logger.add_sink(SinkConfig::default()).unwrap();

    c.bench_function("basic_info_log", |b| {
        b.iter(|| {
            logger
                .info(black_box("Benchmark message".to_string()))
                .unwrap();
        });
    });
    export_bench_result("basic_info_log");
}

fn bench_all_levels(c: &mut Criterion) {
    let logger = Logger::new();
    logger.add_sink(SinkConfig::default()).unwrap();

    let mut group = c.benchmark_group("log_levels");

    group.bench_function("trace", |b| {
        b.iter(|| logger.trace(black_box("Trace".to_string())).unwrap());
    });
    export_bench_result("log_levels/trace");

    group.bench_function("debug", |b| {
        b.iter(|| logger.debug(black_box("Debug".to_string())).unwrap());
    });
    export_bench_result("log_levels/debug");

    group.bench_function("info", |b| {
        b.iter(|| logger.info(black_box("Info".to_string())).unwrap());
    });
    export_bench_result("log_levels/info");

    group.bench_function("warning", |b| {
        b.iter(|| logger.warning(black_box("Warning".to_string())).unwrap());
    });
    export_bench_result("log_levels/warning");

    group.bench_function("error", |b| {
        b.iter(|| logger.error(black_box("Error".to_string())).unwrap());
    });
    export_bench_result("log_levels/error");

    group.finish();
}

fn bench_file_logging(c: &mut Criterion) {
    let temp_dir = TempDir::new().unwrap();
    let log_path = temp_dir.path().join("bench.log");

    let logger = Logger::new();
    let config = SinkConfig {
        path: Some(log_path),
        async_write: true,
        ..Default::default()
    };
    logger.add_sink(config).unwrap();

    c.bench_function("file_logging", |b| {
        b.iter(|| {
            logger
                .info(black_box("File benchmark message".to_string()))
                .unwrap();
        });
    });
    export_bench_result("file_logging");
}

fn bench_with_context(c: &mut Criterion) {
    let logger = Logger::new();
    logger.add_sink(SinkConfig::default()).unwrap();

    logger.bind("user_id".to_string(), serde_json::json!("12345"));
    logger.bind("session".to_string(), serde_json::json!("abc-def"));
    logger.bind("request_id".to_string(), serde_json::json!("req-xyz"));

    c.bench_function("logging_with_context", |b| {
        b.iter(|| {
            logger
                .info(black_box("Message with context".to_string()))
                .unwrap();
        });
    });
    export_bench_result("logging_with_context");
}

fn bench_concurrent_logging(c: &mut Criterion) {
    use std::sync::Arc;
    use std::thread;

    let logger = Arc::new(Logger::new());
    logger.add_sink(SinkConfig::default()).unwrap();

    c.bench_function("concurrent_10_threads", |b| {
        b.iter(|| {
            let mut handles = vec![];

            for i in 0..10 {
                let logger_clone = Arc::clone(&logger);
                let handle = thread::spawn(move || {
                    logger_clone.info(format!("Thread {} message", i)).unwrap();
                });
                handles.push(handle);
            }

            for handle in handles {
                handle.join().unwrap();
            }
        });
    });
    export_bench_result("concurrent_10_threads");
}

fn bench_multiple_sinks(c: &mut Criterion) {
    let mut group = c.benchmark_group("multiple_sinks");

    for sink_count in [1, 2, 5, 10].iter() {
        let logger = Logger::new();

        for _ in 0..*sink_count {
            logger.add_sink(SinkConfig::default()).unwrap();
        }

        group.bench_with_input(
            BenchmarkId::from_parameter(sink_count),
            sink_count,
            |b, _| {
                b.iter(|| {
                    logger
                        .info(black_box("Multi-sink message".to_string()))
                        .unwrap();
                });
            },
        );
        export_bench_result(&format!("multiple_sinks/{}", sink_count));
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_basic_logging,
    bench_all_levels,
    bench_file_logging,
    bench_with_context,
    bench_concurrent_logging,
    bench_multiple_sinks
);

criterion_main!(benches);
