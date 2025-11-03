// Logger configuration with comprehensive settings

use crate::level::{CustomLevel, Level};
use std::collections::HashMap;

#[derive(Clone)]
pub struct LoggerConfig {
    pub level: Level,
    pub color: bool,
    pub global_color_display: bool,
    pub global_console_display: bool,
    pub global_file_storage: bool,
    pub level_colors: HashMap<Level, String>,
    pub custom_levels: HashMap<String, CustomLevel>,
    pub json: bool,
    pub pretty_json: bool,
    pub console: bool,
    pub show_time: bool,
    pub show_module: bool,
    pub show_function: bool,
    pub show_filename: bool,
    pub show_lineno: bool,
    pub console_levels: HashMap<Level, bool>,
    pub time_levels: HashMap<Level, bool>,
    pub color_levels: HashMap<Level, bool>,
    pub storage_levels: HashMap<Level, bool>,
    pub auto_sink: bool,
    pub log_compact: bool,
    pub enable_gpu: bool,
    pub gpu_buffer_size: usize,
    pub enable_callbacks: bool,
    pub enable_exception_handling: bool,
    pub enable_version_check: bool,
    pub debug_mode: bool,
    pub debug_log_file: Option<std::path::PathBuf>,
}

impl Default for LoggerConfig {
    fn default() -> Self {
        let mut level_colors = HashMap::new();
        for level in Level::all_levels() {
            level_colors.insert(level, level.default_color().to_string());
        }

        Self {
            level: Level::Info,
            color: true,
            global_color_display: true,
            global_console_display: true,
            global_file_storage: true,
            level_colors,
            custom_levels: HashMap::new(),
            json: false,
            pretty_json: false,
            console: true,
            show_time: true,
            show_module: true,
            show_function: true,
            show_filename: false,
            show_lineno: false,
            console_levels: HashMap::new(),
            time_levels: HashMap::new(),
            color_levels: HashMap::new(),
            storage_levels: HashMap::new(),
            auto_sink: true,
            log_compact: false,
            enable_gpu: false,
            gpu_buffer_size: 1024 * 1024,
            enable_callbacks: true,
            enable_exception_handling: true,
            enable_version_check: true,
            debug_mode: false,
            debug_log_file: None,
        }
    }
}

impl LoggerConfig {
    pub fn add_custom_level(
        &mut self,
        name: String,
        priority: u8,
        color: String,
    ) -> Result<(), crate::error::LoglyError> {
        if self.custom_levels.contains_key(&name) {
            return Err(crate::error::LoglyError::CustomLevelExists(name));
        }
        self.custom_levels
            .insert(name.clone(), CustomLevel::new(name, priority, color));
        Ok(())
    }

    pub fn remove_custom_level(&mut self, name: &str) -> bool {
        self.custom_levels.remove(name).is_some()
    }

    pub fn get_custom_level(&self, name: &str) -> Option<&CustomLevel> {
        self.custom_levels.get(name)
    }
}
