// Configuration file support (logly.toml)

use crate::config::LoggerConfig;
use crate::error::{LoglyError, Result};
use crate::level::Level;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct ConfigFile {
    #[serde(default)]
    pub logly: Option<LoglyConfig>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct LoglyConfig {
    #[serde(default)]
    pub configuration: Option<ConfigurationSection>,
    #[serde(default)]
    pub display: Option<DisplaySection>,
    #[serde(default)]
    pub format: Option<FormatSection>,
    #[serde(default)]
    pub sinks: Option<SinksSection>,
    #[serde(default)]
    pub gpu: Option<GpuSection>,
    #[serde(default)]
    pub features: Option<FeaturesSection>,
    #[serde(default)]
    pub debug: Option<DebugSection>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct ConfigurationSection {
    pub level: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct DisplaySection {
    pub global_color_display: Option<bool>,
    pub global_console_display: Option<bool>,
    pub global_file_storage: Option<bool>,
    pub color: Option<bool>,
    pub console: Option<bool>,
    pub show_time: Option<bool>,
    pub show_module: Option<bool>,
    pub show_function: Option<bool>,
    pub show_filename: Option<bool>,
    pub show_lineno: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct FormatSection {
    pub json: Option<bool>,
    pub pretty_json: Option<bool>,
    pub log_compact: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct SinksSection {
    pub auto_sink: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct GpuSection {
    pub enable_gpu: Option<bool>,
    pub gpu_buffer_size: Option<usize>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct FeaturesSection {
    pub enable_callbacks: Option<bool>,
    pub enable_exception_handling: Option<bool>,
    pub enable_version_check: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct DebugSection {
    pub debug_mode: Option<bool>,
    pub debug_log_file: Option<String>,
}

pub struct ConfigFileLoader {
    scan_enabled: bool,
    custom_path: Option<PathBuf>,
}

impl ConfigFileLoader {
    pub fn new() -> Self {
        Self {
            scan_enabled: true,
            custom_path: None,
        }
    }

    pub fn disable_scan(&mut self) {
        self.scan_enabled = false;
    }

    pub fn set_custom_path(&mut self, path: PathBuf) {
        self.custom_path = Some(path);
    }

    pub fn load(&self) -> Result<Option<LoggerConfig>> {
        if !self.scan_enabled && self.custom_path.is_none() {
            return Ok(None);
        }

        let config_path = if let Some(ref path) = self.custom_path {
            if !path.exists() {
                return Err(LoglyError::InvalidConfig(format!(
                    "Custom config file not found: {}",
                    path.display()
                )));
            }
            path.clone()
        } else {
            let default_path = PathBuf::from("logly.toml");
            if !default_path.exists() {
                return Ok(None);
            }
            default_path
        };

        // Check for duplicate config files
        if self.custom_path.is_none() {
            let mut found_configs = Vec::new();
            for name in &["logly.toml", "Logly.toml", "LOGLY.toml"] {
                if Path::new(name).exists() {
                    found_configs.push(name.to_string());
                }
            }
            if found_configs.len() > 1 {
                eprintln!(
                    "⚠️  Warning: Multiple config files found: {:?}. Using: {}",
                    found_configs, found_configs[0]
                );
            }
        }

        let content = fs::read_to_string(&config_path).map_err(|e| {
            LoglyError::InvalidConfig(format!("Failed to read config file: {}", e))
        })?;

        let config_file: ConfigFile = toml::from_str(&content).map_err(|e| {
            LoglyError::InvalidConfig(format!("Failed to parse config file: {}", e))
        })?;

        Ok(Some(self.apply_config(config_file)?))
    }

    fn apply_config(&self, file: ConfigFile) -> Result<LoggerConfig> {
        let mut config = LoggerConfig::default();

        if let Some(logly) = file.logly {
            // Configuration section
            if let Some(cfg) = logly.configuration {
                if let Some(level_str) = cfg.level {
                    config.level = level_str.parse::<Level>()?;
                }
            }

            // Display section
            if let Some(display) = logly.display {
                if let Some(v) = display.global_color_display {
                    config.global_color_display = v;
                }
                if let Some(v) = display.global_console_display {
                    config.global_console_display = v;
                }
                if let Some(v) = display.global_file_storage {
                    config.global_file_storage = v;
                }
                if let Some(v) = display.color {
                    config.color = v;
                }
                if let Some(v) = display.console {
                    config.console = v;
                }
                if let Some(v) = display.show_time {
                    config.show_time = v;
                }
                if let Some(v) = display.show_module {
                    config.show_module = v;
                }
                if let Some(v) = display.show_function {
                    config.show_function = v;
                }
                if let Some(v) = display.show_filename {
                    config.show_filename = v;
                }
                if let Some(v) = display.show_lineno {
                    config.show_lineno = v;
                }
            }

            // Format section
            if let Some(format) = logly.format {
                if let Some(v) = format.json {
                    config.json = v;
                }
                if let Some(v) = format.pretty_json {
                    config.pretty_json = v;
                }
                if let Some(v) = format.log_compact {
                    config.log_compact = v;
                }
            }

            // Sinks section
            if let Some(sinks) = logly.sinks {
                if let Some(v) = sinks.auto_sink {
                    config.auto_sink = v;
                }
            }

            // GPU section
            if let Some(gpu) = logly.gpu {
                if let Some(v) = gpu.enable_gpu {
                    config.enable_gpu = v;
                }
                if let Some(v) = gpu.gpu_buffer_size {
                    config.gpu_buffer_size = v;
                }
            }

            // Features section
            if let Some(features) = logly.features {
                if let Some(v) = features.enable_callbacks {
                    config.enable_callbacks = v;
                }
                if let Some(v) = features.enable_exception_handling {
                    config.enable_exception_handling = v;
                }
                if let Some(v) = features.enable_version_check {
                    config.enable_version_check = v;
                }
            }

            // Debug section
            if let Some(debug) = logly.debug {
                if let Some(v) = debug.debug_mode {
                    config.debug_mode = v;
                }
                if let Some(path) = debug.debug_log_file {
                    config.debug_log_file = Some(PathBuf::from(path));
                }
            }
        }

        Ok(config)
    }
}

impl Default for ConfigFileLoader {
    fn default() -> Self {
        Self::new()
    }
}
