//! Main logger implementation
//!
//! Provides the primary Logger struct with support for multiple sinks, callbacks,
//! context binding, GPU acceleration, and comprehensive configuration options.

use crate::callback::CallbackManager;
use crate::config::LoggerConfig;
use crate::config_file::ConfigFileLoader;
use crate::error::{LoglyError, Result};
use crate::gpu::GpuLogger;
use crate::level::Level;
use crate::record::LogRecord;
use crate::sink::{Sink, SinkConfig};
use crate::version::VersionChecker;
use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::Arc;

pub struct Logger {
    config: Arc<RwLock<LoggerConfig>>,
    sinks: Arc<RwLock<HashMap<usize, Arc<Sink>>>>,
    next_sink_id: Arc<RwLock<usize>>,
    enabled: Arc<RwLock<bool>>,
    bound_fields: Arc<RwLock<HashMap<String, serde_json::Value>>>,
    callbacks: Arc<CallbackManager>,
    gpu_logger: Arc<RwLock<Option<GpuLogger>>>,
    version_checker: Arc<RwLock<VersionChecker>>,
    auto_sink_initialized: Arc<RwLock<bool>>,
    config_file_loader: Arc<RwLock<ConfigFileLoader>>,
}

impl Logger {
    pub fn new() -> Self {
        let config_loader = ConfigFileLoader::new();
        let file_config = config_loader.load().ok().flatten();
        
        let initial_config = file_config.unwrap_or_default();
        
        let logger = Self {
            config: Arc::new(RwLock::new(initial_config.clone())),
            sinks: Arc::new(RwLock::new(HashMap::new())),
            next_sink_id: Arc::new(RwLock::new(1)),
            enabled: Arc::new(RwLock::new(true)),
            bound_fields: Arc::new(RwLock::new(HashMap::new())),
            callbacks: Arc::new(CallbackManager::new()),
            gpu_logger: Arc::new(RwLock::new(None)),
            version_checker: Arc::new(RwLock::new(VersionChecker::default())),
            auto_sink_initialized: Arc::new(RwLock::new(false)),
            config_file_loader: Arc::new(RwLock::new(config_loader)),
        };

        // Check for updates on initialization
        if initial_config.enable_version_check {
            if let Ok(Some(msg)) = logger.version_checker.read().check_for_updates() {
                eprintln!("{}", msg);
            }
        }

        logger
    }
    
    pub fn with_config_file(path: std::path::PathBuf) -> Result<Self> {
        let mut config_loader = ConfigFileLoader::new();
        config_loader.set_custom_path(path);
        let file_config = config_loader.load()?.unwrap_or_default();
        
        let logger = Self {
            config: Arc::new(RwLock::new(file_config.clone())),
            sinks: Arc::new(RwLock::new(HashMap::new())),
            next_sink_id: Arc::new(RwLock::new(1)),
            enabled: Arc::new(RwLock::new(true)),
            bound_fields: Arc::new(RwLock::new(HashMap::new())),
            callbacks: Arc::new(CallbackManager::new()),
            gpu_logger: Arc::new(RwLock::new(None)),
            version_checker: Arc::new(RwLock::new(VersionChecker::default())),
            auto_sink_initialized: Arc::new(RwLock::new(false)),
            config_file_loader: Arc::new(RwLock::new(config_loader)),
        };

        if file_config.enable_version_check {
            if let Ok(Some(msg)) = logger.version_checker.read().check_for_updates() {
                eprintln!("{}", msg);
            }
        }

        Ok(logger)
    }
    
    pub fn disable_config_file_scan(&self) {
        self.config_file_loader.write().disable_scan();
    }

    pub fn configure(&self, config: LoggerConfig) {
        let enable_gpu = config.enable_gpu;
        let gpu_buffer_size = config.gpu_buffer_size;
        let enable_version_check = config.enable_version_check;
        let auto_sink = config.auto_sink;
        let debug_mode = config.debug_mode;

        // Validate configuration
        if gpu_buffer_size == 0 {
            eprintln!("[LOGLY WARNING] GPU buffer size is 0, using default 1MB");
        }
        
        if config.custom_levels.len() > 100 {
            eprintln!("[LOGLY WARNING] Too many custom levels ({}), may impact performance", config.custom_levels.len());
        }

        *self.config.write() = config;

        // Initialize GPU if enabled
        if enable_gpu {
            match GpuLogger::new(gpu_buffer_size) {
                Ok(gpu) => {
                    *self.gpu_logger.write() = Some(gpu);
                    if debug_mode {
                        eprintln!("[LOGLY DEBUG] GPU logging initialized");
                    }
                }
                Err(e) => {
                    eprintln!("[LOGLY WARNING] Failed to initialize GPU: {}", e);
                    eprintln!("[LOGLY INFO] Continuing with CPU-only logging");
                }
            }
        }

        // Enable version checker
        if enable_version_check {
            self.version_checker.write().enable();
        }

        // Initialize auto-sink
        if auto_sink && !*self.auto_sink_initialized.read() {
            if let Err(e) = self.initialize_auto_sink() {
                eprintln!("[LOGLY WARNING] Auto-sink initialization failed: {}", e);
                if debug_mode {
                    eprintln!("[LOGLY DEBUG] Auto-sink initialization failed: {}", e);
                }
            }
        }
    }

    fn initialize_auto_sink(&self) -> Result<()> {
        let config = SinkConfig::default();
        self.add_sink(config)?;
        *self.auto_sink_initialized.write() = true;
        
        if self.config.read().debug_mode {
            eprintln!("[LOGLY DEBUG] Auto-sink initialized");
        }
        Ok(())
    }

    pub fn add_sink(&self, mut config: SinkConfig) -> Result<usize> {
        let sink_count = self.sinks.read().len();
        if sink_count >= 50 {
            eprintln!("[LOGLY WARNING] High number of sinks ({}), may impact performance", sink_count);
        }

        let mut next_id = self.next_sink_id.write();
        let id = *next_id;
        *next_id += 1;

        // Apply global color settings if not explicitly set
        let logger_config = self.config.read();
        if config.color && !logger_config.global_color_display {
            config.color = false;
        }
        drop(logger_config);

        let mut sink = match Sink::new(id, config) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("[LOGLY ERROR] Failed to create sink: {}", e);
                return Err(e);
            }
        };
        
        // Apply custom level colors from logger config
        let level_colors = self.config.read().level_colors.clone();
        sink.set_level_colors(level_colors);
        
        self.sinks.write().insert(id, Arc::new(sink));

        if self.config.read().debug_mode {
            eprintln!("[LOGLY DEBUG] Sink {} added successfully", id);
        }

        Ok(id)
    }

    pub fn remove_sink(&self, id: usize) -> bool {
        self.sinks.write().remove(&id).is_some()
    }

    pub fn remove_all_sinks(&self) -> usize {
        let mut sinks = self.sinks.write();
        let count = sinks.len();
        sinks.clear();
        count
    }

    pub fn enable(&self) {
        *self.enabled.write() = true;
    }

    pub fn disable(&self) {
        *self.enabled.write() = false;
    }

    pub fn bind(&self, key: String, value: serde_json::Value) {
        if self.config.read().debug_mode {
            eprintln!("[LOGLY DEBUG] Bound field: {} = {:?}", key, value);
        }
        self.bound_fields.write().insert(key, value);
    }

    pub fn unbind(&self, key: &str) -> Option<serde_json::Value> {
        self.bound_fields.write().remove(key)
    }

    pub fn clear_bindings(&self) {
        self.bound_fields.write().clear();
    }

    pub fn log(&self, level: Level, message: String) -> Result<()> {
        if !*self.enabled.read() {
            return Ok(());
        }

        let config = self.config.read();
        if level < config.level {
            return Ok(());
        }

        let debug_mode = config.debug_mode;
        let debug_log_file = config.debug_log_file.clone();
        let global_console = config.global_console_display;
        let global_storage = config.global_file_storage;
        drop(config);

        // If global console display is false, don't log anywhere
        if !global_console && !global_storage {
            return Ok(());
        }

        let mut record = LogRecord::new(level, message.clone());

        for (key, value) in self.bound_fields.read().iter() {
            record.fields.insert(key.clone(), value.clone());
        }

        // Debug logging
        if debug_mode {
            let debug_msg = format!("[LOGLY DEBUG] Logging: {} - {}", level.as_str(), message);
            if let Some(ref path) = debug_log_file {
                use std::fs::OpenOptions;
                use std::io::Write;
                if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(path) {
                    let _ = writeln!(file, "{}", debug_msg);
                }
            } else {
                eprintln!("{}", debug_msg);
            }
        }

        // Execute callbacks
        if self.config.read().enable_callbacks {
            let errors = self.callbacks.execute_log_callbacks(&record);
            for error in errors {
                if debug_mode {
                    eprintln!("[LOGLY DEBUG] Callback error: {}", error);
                }
            }
        }

        // Write to GPU if enabled
        if let Some(ref gpu) = *self.gpu_logger.read() {
            if gpu.is_enabled() {
                let data = format!("{:?}", record).into_bytes();
                if let Err(e) = gpu.write_to_gpu(&data) {
                    if debug_mode {
                        eprintln!("[LOGLY DEBUG] GPU write error: {}", e);
                    }
                }
            }
        }

        // Write to sinks based on global settings
        let sinks = self.sinks.read();
        for sink in sinks.values() {
            if let Err(e) = sink.log(&record, global_console, global_storage) {
                if self.config.read().enable_exception_handling {
                    self.handle_exception(&format!("Sink error: {}", e));
                } else {
                    return Err(e);
                }
            }
        }

        Ok(())
    }

    pub fn log_custom(&self, level_name: &str, message: String) -> Result<()> {
        let config = self.config.read();
        if let Some(custom_level) = config.get_custom_level(level_name) {
            let priority = custom_level.priority;
            drop(config);
            
            // Use closest standard level based on priority
            let level = Level::from_priority(priority).unwrap_or(Level::Info);
            self.log(level, message)
        } else {
            Err(LoglyError::InvalidLevel(level_name.to_string()))
        }
    }

    fn handle_exception(&self, error: &str) {
        let backtrace = backtrace::Backtrace::new();
        let backtrace_str = format!("{:?}", backtrace);
        
        self.callbacks.execute_exception_callbacks(error, &backtrace_str);
        
        if self.config.read().debug_mode {
            eprintln!("[LOGLY EXCEPTION] {}\n{}", error, backtrace_str);
        }
    }

    pub fn trace(&self, message: String) -> Result<()> {
        self.log(Level::Trace, message)
    }

    pub fn debug(&self, message: String) -> Result<()> {
        self.log(Level::Debug, message)
    }

    pub fn info(&self, message: String) -> Result<()> {
        self.log(Level::Info, message)
    }

    pub fn success(&self, message: String) -> Result<()> {
        self.log(Level::Success, message)
    }

    pub fn warning(&self, message: String) -> Result<()> {
        self.log(Level::Warning, message)
    }

    pub fn error(&self, message: String) -> Result<()> {
        self.log(Level::Error, message)
    }

    pub fn critical(&self, message: String) -> Result<()> {
        self.log(Level::Critical, message)
    }

    pub fn fail(&self, message: String) -> Result<()> {
        self.log(Level::Fail, message)
    }

    // Callback management
    pub fn add_log_callback<F>(&self, callback: F)
    where
        F: Fn(&LogRecord) -> std::result::Result<(), String> + Send + Sync + 'static,
    {
        self.callbacks.add_log_callback(Arc::new(callback));
    }

    pub fn add_color_callback<F>(&self, callback: F)
    where
        F: Fn(Level, &str) -> String + Send + Sync + 'static,
    {
        self.callbacks.add_color_callback(Arc::new(callback));
    }

    pub fn add_exception_callback<F>(&self, callback: F)
    where
        F: Fn(&str, &str) -> () + Send + Sync + 'static,
    {
        self.callbacks.add_exception_callback(Arc::new(callback));
    }

    pub fn clear_callbacks(&self) {
        self.callbacks.clear_all();
    }

    // GPU management
    pub fn enable_gpu(&self) -> Result<()> {
        if let Some(ref gpu) = *self.gpu_logger.read() {
            gpu.enable()
        } else {
            let mut config = self.config.write();
            config.enable_gpu = true;
            let gpu_buffer_size = config.gpu_buffer_size;
            drop(config);

            let gpu = GpuLogger::new(gpu_buffer_size)?;
            gpu.enable()?;
            *self.gpu_logger.write() = Some(gpu);
            Ok(())
        }
    }

    pub fn disable_gpu(&self) {
        if let Some(ref gpu) = *self.gpu_logger.read() {
            gpu.disable();
        }
    }

    pub fn gpu_info(&self) -> String {
        if let Some(ref gpu) = *self.gpu_logger.read() {
            gpu.get_info()
        } else {
            "GPU logging not initialized".to_string()
        }
    }

    // Custom level management
    pub fn add_custom_level(&self, name: String, priority: u8, color: String) -> Result<()> {
        if priority < 5 || priority > 50 {
            eprintln!("[LOGLY WARNING] Custom level priority {} is outside standard range (5-50)", priority);
        }
        self.config.write().add_custom_level(name, priority, color)
    }

    pub fn remove_custom_level(&self, name: &str) -> bool {
        self.config.write().remove_custom_level(name)
    }

    // Sink management helpers
    pub fn get_sink_count(&self) -> usize {
        self.sinks.read().len()
    }

    pub fn list_sinks(&self) -> Vec<usize> {
        self.sinks.read().keys().copied().collect()
    }

    // Debug mode
    pub fn enable_debug(&self) {
        self.config.write().debug_mode = true;
        eprintln!("[LOGLY DEBUG] Debug mode enabled");
    }

    pub fn disable_debug(&self) {
        self.config.write().debug_mode = false;
    }

    // Version checking
    pub fn check_version(&self) -> Result<Option<String>> {
        self.version_checker.read().check_for_updates()
    }

    pub fn current_version(&self) -> &'static str {
        VersionChecker::current_version()
    }
}

impl Default for Logger {
    fn default() -> Self {
        Self::new()
    }
}
