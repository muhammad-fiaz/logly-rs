//! GPU/CUDA acceleration for high-performance logging
//!
//! Provides optional GPU acceleration using CUDA for high-throughput logging scenarios.
//! Requires the `gpu` feature flag and CUDA toolkit to be installed.
//!
//! # Example
//!
//! ```no_run
//! use logly::GpuLogger;
//!
//! let gpu = GpuLogger::new(1024 * 1024)?; // 1MB buffer
//! gpu.enable()?;
//! # Ok::<(), logly::LoglyError>(())
//! ```

#[cfg(feature = "gpu")]
use cudarc::driver::{CudaDevice, CudaSlice};

use crate::error::{LoglyError, Result};
use parking_lot::RwLock;
use std::sync::Arc;

/// GPU logger for CUDA-accelerated logging operations.
///
/// Manages GPU device initialization, buffer allocation, and data transfer.
/// Falls back gracefully to CPU-only logging if GPU is unavailable.
pub struct GpuLogger {
    /// CUDA device handle (only available with gpu feature)
    #[cfg(feature = "gpu")]
    device: Option<CudaDevice>,
    /// GPU memory buffer for log data (only available with gpu feature)
    #[cfg(feature = "gpu")]
    buffer: Arc<RwLock<Option<CudaSlice<u8>>>>,
    /// Whether GPU logging is currently enabled
    enabled: Arc<RwLock<bool>>,
    /// Size of the GPU buffer in bytes
    #[allow(dead_code)]
    buffer_size: usize,
}

impl GpuLogger {
    /// Creates a new GPU logger with the specified buffer size.
    ///
    /// # Arguments
    ///
    /// * `buffer_size` - Size of the GPU buffer in bytes
    ///
    /// # Returns
    ///
    /// A new GpuLogger instance, or an error if initialization fails
    pub fn new(buffer_size: usize) -> Result<Self> {
        #[cfg(feature = "gpu")]
        {
            let device = match CudaDevice::new(0) {
                Ok(dev) => Some(dev),
                Err(e) => {
                    eprintln!("Warning: Failed to initialize CUDA device: {:?}", e);
                    None
                }
            };

            Ok(Self {
                device,
                buffer: Arc::new(RwLock::new(None)),
                enabled: Arc::new(RwLock::new(device.is_some())),
                buffer_size,
            })
        }

        #[cfg(not(feature = "gpu"))]
        {
            Ok(Self {
                enabled: Arc::new(RwLock::new(false)),
                buffer_size,
            })
        }
    }

    /// Checks if GPU acceleration is available.
    ///
    /// # Returns
    ///
    /// `true` if CUDA device is initialized and available, `false` otherwise
    pub fn is_available(&self) -> bool {
        #[cfg(feature = "gpu")]
        {
            self.device.is_some()
        }

        #[cfg(not(feature = "gpu"))]
        {
            false
        }
    }

    /// Checks if GPU logging is currently enabled.
    ///
    /// # Returns
    ///
    /// `true` if GPU logging is enabled, `false` otherwise
    pub fn is_enabled(&self) -> bool {
        *self.enabled.read()
    }

    /// Enables GPU logging.
    ///
    /// # Returns
    ///
    /// An error if GPU is not available or initialization fails
    pub fn enable(&self) -> Result<()> {
        #[cfg(feature = "gpu")]
        {
            if self.device.is_none() {
                return Err(LoglyError::GpuError(
                    "CUDA device not available".to_string(),
                ));
            }
            *self.enabled.write() = true;
            Ok(())
        }

        #[cfg(not(feature = "gpu"))]
        {
            Err(LoglyError::GpuError(
                "GPU feature not enabled. Compile with --features gpu".to_string(),
            ))
        }
    }

    /// Disables GPU logging.
    pub fn disable(&self) {
        *self.enabled.write() = false;
    }

    /// Allocates GPU buffer for log data (only available with gpu feature).
    ///
    /// # Returns
    ///
    /// An error if buffer allocation fails
    #[cfg(feature = "gpu")]
    pub fn allocate_buffer(&self) -> Result<()> {
        if let Some(ref device) = self.device {
            match device.alloc_zeros::<u8>(self.buffer_size) {
                Ok(buffer) => {
                    *self.buffer.write() = Some(buffer);
                    Ok(())
                }
                Err(e) => Err(LoglyError::GpuError(format!(
                    "Failed to allocate GPU buffer: {:?}",
                    e
                ))),
            }
        } else {
            Err(LoglyError::GpuError(
                "CUDA device not available".to_string(),
            ))
        }
    }

    /// Writes log data to GPU memory (only available with gpu feature).
    ///
    /// # Arguments
    ///
    /// * `data` - Byte slice to write to GPU
    ///
    /// # Returns
    ///
    /// An error if GPU write fails
    #[cfg(feature = "gpu")]
    pub fn write_to_gpu(&self, data: &[u8]) -> Result<()> {
        if !self.is_enabled() {
            return Ok(());
        }

        if let Some(ref device) = self.device {
            let buffer = self.buffer.read();
            if buffer.is_none() {
                drop(buffer);
                self.allocate_buffer()?;
            }

            // Copy data to GPU
            match device.htod_copy(data.to_vec()) {
                Ok(_) => Ok(()),
                Err(e) => Err(LoglyError::GpuError(format!(
                    "Failed to copy to GPU: {:?}",
                    e
                ))),
            }
        } else {
            Err(LoglyError::GpuError(
                "CUDA device not available".to_string(),
            ))
        }
    }

    /// Writes log data to GPU memory (stub when gpu feature is disabled).
    ///
    /// # Returns
    ///
    /// An error indicating GPU feature is not enabled
    #[cfg(not(feature = "gpu"))]
    pub fn write_to_gpu(&self, _data: &[u8]) -> Result<()> {
        Err(LoglyError::GpuError("GPU feature not enabled".to_string()))
    }

    /// Returns information about GPU logging status.
    ///
    /// # Returns
    ///
    /// A string describing GPU device, buffer size, and status
    pub fn get_info(&self) -> String {
        #[cfg(feature = "gpu")]
        {
            if let Some(ref _device) = self.device {
                format!(
                    "GPU Logging: Enabled\nDevice: CUDA Device 0\nBuffer Size: {} bytes\nStatus: {}",
                    self.buffer_size,
                    if self.is_enabled() { "Active" } else { "Inactive" }
                )
            } else {
                "GPU Logging: Not Available (CUDA device initialization failed)".to_string()
            }
        }

        #[cfg(not(feature = "gpu"))]
        {
            "GPU Logging: Not Available (compile with --features gpu)".to_string()
        }
    }
}

impl Default for GpuLogger {
    fn default() -> Self {
        Self::new(1024 * 1024).unwrap_or_else(|_| {
            #[cfg(feature = "gpu")]
            {
                Self {
                    device: None,
                    buffer: Arc::new(RwLock::new(None)),
                    enabled: Arc::new(RwLock::new(false)),
                    buffer_size: 1024 * 1024,
                }
            }
            #[cfg(not(feature = "gpu"))]
            {
                Self {
                    enabled: Arc::new(RwLock::new(false)),
                    buffer_size: 1024 * 1024,
                }
            }
        })
    }
}
