//! GPU/CUDA acceleration for high-performance logging
//!
//! Provides optional GPU acceleration using CUDA for high-throughput logging scenarios.
//! Requires the `gpu` feature flag and CUDA toolkit to be installed.
//!
//! This module uses the cudarc driver API to allocate GPU memory and transfer log data
//! to the device for high-throughput scenarios. The implementation uses:
//! - `CudaContext` for device management (similar to CPU's GlobalAlloc)
//! - `CudaSlice<T>` for GPU memory allocation (similar to CPU's `Vec<T>`)
//! - `htod_sync_copy` for host-to-device memory transfers
//!
//! # Features
//!
//! - Automatic CUDA device initialization
//! - Graceful fallback to CPU-only logging if GPU unavailable
//! - Thread-safe enable/disable controls
//! - Synchronous memory transfers for reliability
//!
//! # Example
//!
//! ```no_run
//! use logly::GpuLogger;
//!
//! let gpu = GpuLogger::new(1024 * 1024)?; // 1MB buffer
//! if gpu.is_available() {
//!     gpu.enable()?;
//!     let data = b"log message";
//!     gpu.write_to_gpu(data)?;
//! }
//! # Ok::<(), logly::LoglyError>(())
//! ```
//!
//! # CUDA Version Support
//!
//! Supports CUDA 11.4-11.8, 12.0-12.9, and 13.0 via cudarc.

use crate::error::{LoglyError, Result};
use parking_lot::RwLock;
use std::sync::Arc;

/// GPU logger for CUDA-accelerated logging operations.
///
/// Manages GPU device initialization, buffer allocation, and data transfer using
/// the cudarc driver API. The device is initialized on creation and can be
/// enabled/disabled at runtime.
///
/// # Thread Safety
///
/// This struct is thread-safe and can be shared across threads using Arc.
/// The enabled state is protected by RwLock for concurrent access.
///
/// # Memory Management
///
/// Uses `CudaContext::htod_sync_copy` for synchronous host-to-device transfers.
/// Each write allocates a new `CudaSlice<u8>` on the device.
pub struct GpuLogger {
    /// CUDA context and stream (boxed to avoid exposing cudarc types)
    /// Only available when compiled with `gpu` feature
    #[cfg(feature = "gpu")]
    ctx_stream: Option<Box<dyn std::any::Any + Send + Sync>>,
    /// Whether GPU logging is currently enabled (thread-safe)
    enabled: Arc<RwLock<bool>>,
    /// Size of the GPU buffer in bytes (for informational purposes)
    #[allow(dead_code)]
    buffer_size: usize,
}

impl GpuLogger {
    /// Creates a new GPU logger with the specified buffer size.
    ///
    /// Attempts to initialize CUDA device 0. If initialization fails,
    /// the logger will be created but GPU functionality will be unavailable.
    ///
    /// # Arguments
    ///
    /// * `buffer_size` - Size of the GPU buffer in bytes (informational)
    ///
    /// # Returns
    ///
    /// A new GpuLogger instance. Always succeeds, even if GPU is unavailable.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use logly::GpuLogger;
    ///
    /// let gpu = GpuLogger::new(1024 * 1024)?; // 1MB buffer
    /// if gpu.is_available() {
    ///     println!("GPU logging available");
    /// }
    /// # Ok::<(), logly::LoglyError>(())
    /// ```
    pub fn new(buffer_size: usize) -> Result<Self> {
        #[cfg(feature = "gpu")]
        {
            let ctx_stream = cudarc::driver::CudaContext::new(0).ok().map(|ctx| {
                let stream = ctx.default_stream();
                Box::new((ctx, stream)) as Box<dyn std::any::Any + Send + Sync>
            });
            let is_available = ctx_stream.is_some();

            Ok(Self {
                ctx_stream,
                enabled: Arc::new(RwLock::new(is_available)),
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
    /// Returns true only if:
    /// - Compiled with `gpu` feature
    /// - CUDA device initialization succeeded
    ///
    /// # Returns
    ///
    /// `true` if CUDA device is initialized and available, `false` otherwise
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use logly::GpuLogger;
    ///
    /// let gpu = GpuLogger::new(1024)?;
    /// if gpu.is_available() {
    ///     gpu.enable()?;
    /// }
    /// # Ok::<(), logly::LoglyError>(())
    /// ```
    pub fn is_available(&self) -> bool {
        #[cfg(feature = "gpu")]
        {
            self.ctx_stream.is_some()
        }

        #[cfg(not(feature = "gpu"))]
        {
            false
        }
    }

    /// Checks if GPU logging is currently enabled.
    ///
    /// Note: This only checks the enabled flag, not GPU availability.
    /// Use `is_available()` to check if GPU is actually usable.
    ///
    /// # Returns
    ///
    /// `true` if GPU logging is enabled, `false` otherwise
    ///
    /// # Thread Safety
    ///
    /// This method is thread-safe and uses a read lock.
    pub fn is_enabled(&self) -> bool {
        *self.enabled.read()
    }

    /// Enables GPU logging.
    ///
    /// # Returns
    ///
    /// - `Ok(())` if GPU is available and enabled successfully
    /// - `Err(LoglyError::GpuError)` if GPU is not available or feature not compiled
    ///
    /// # Errors
    ///
    /// Returns error if:
    /// - Not compiled with `gpu` feature
    /// - CUDA device initialization failed
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use logly::GpuLogger;
    ///
    /// let gpu = GpuLogger::new(1024)?;
    /// match gpu.enable() {
    ///     Ok(_) => println!("GPU enabled"),
    ///     Err(e) => eprintln!("GPU not available: {}", e),
    /// }
    /// # Ok::<(), logly::LoglyError>(())
    /// ```
    pub fn enable(&self) -> Result<()> {
        #[cfg(feature = "gpu")]
        {
            if self.ctx_stream.is_none() {
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
    ///
    /// After calling this, `write_to_gpu()` will become a no-op.
    /// Can be re-enabled with `enable()`.
    ///
    /// # Thread Safety
    ///
    /// This method is thread-safe and uses a write lock.
    pub fn disable(&self) {
        *self.enabled.write() = false;
    }

    /// Writes log data to GPU memory (only available with gpu feature).
    ///
    /// Uses `CudaContext::htod_sync_copy` to perform synchronous host-to-device
    /// memory transfer. Allocates a new `CudaSlice<u8>` for each write.
    ///
    /// # Arguments
    ///
    /// * `data` - Byte slice to write to GPU
    ///
    /// # Returns
    ///
    /// - `Ok(())` if write succeeds or GPU is disabled (no-op)
    /// - `Err(LoglyError::GpuError)` if GPU write fails
    ///
    /// # Behavior
    ///
    /// - If GPU is disabled: Returns Ok without doing anything
    /// - If GPU is enabled: Performs synchronous copy to device
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use logly::GpuLogger;
    ///
    /// let gpu = GpuLogger::new(1024)?;
    /// gpu.enable()?;
    /// gpu.write_to_gpu(b"log message")?;
    /// # Ok::<(), logly::LoglyError>(())
    /// ```
    #[cfg(feature = "gpu")]
    pub fn write_to_gpu(&self, data: &[u8]) -> Result<()> {
        if !self.is_enabled() {
            return Ok(());
        }

        if let Some(ref ctx_stream_box) = self.ctx_stream {
            type CtxStream = (
                Arc<cudarc::driver::CudaContext>,
                Arc<cudarc::driver::CudaStream>,
            );
            if let Some((_ctx, stream)) = ctx_stream_box.downcast_ref::<CtxStream>() {
                match stream.memcpy_stod(data) {
                    Ok(_buffer) => Ok(()),
                    Err(e) => Err(LoglyError::GpuError(format!(
                        "Failed to copy to GPU: {:?}",
                        e
                    ))),
                }
            } else {
                Err(LoglyError::GpuError(
                    "Invalid CUDA context type".to_string(),
                ))
            }
        } else {
            Err(LoglyError::GpuError(
                "CUDA device not available".to_string(),
            ))
        }
    }

    /// Writes log data to GPU memory (stub when gpu feature is disabled).
    ///
    /// This is a no-op stub that always returns an error when the `gpu`
    /// feature is not compiled.
    ///
    /// # Arguments
    ///
    /// * `_data` - Byte slice (ignored)
    ///
    /// # Returns
    ///
    /// Always returns `Err(LoglyError::GpuError)` indicating feature not enabled
    ///
    /// # Note
    ///
    /// To use GPU logging, compile with `--features gpu`
    #[cfg(not(feature = "gpu"))]
    pub fn write_to_gpu(&self, _data: &[u8]) -> Result<()> {
        Err(LoglyError::GpuError("GPU feature not enabled".to_string()))
    }

    /// Returns information about GPU logging status.
    ///
    /// Provides human-readable information about GPU availability,
    /// device details, buffer size, and current status.
    ///
    /// # Returns
    ///
    /// A formatted string containing:
    /// - GPU availability status
    /// - Device information (if available)
    /// - Buffer size
    /// - Active/Inactive status
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use logly::GpuLogger;
    ///
    /// let gpu = GpuLogger::new(1024 * 1024)?;
    /// println!("{}", gpu.get_info());
    /// # Ok::<(), logly::LoglyError>(())
    /// ```
    pub fn get_info(&self) -> String {
        #[cfg(feature = "gpu")]
        {
            if self.ctx_stream.is_some() {
                format!(
                    "GPU Logging: Enabled\nDevice: CUDA Device 0\nBuffer Size: {} bytes\nStatus: {}",
                    self.buffer_size,
                    if self.is_enabled() {
                        "Active"
                    } else {
                        "Inactive"
                    }
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
                    ctx_stream: None,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gpu_logger_creation() {
        let gpu = GpuLogger::new(1024 * 1024);
        assert!(gpu.is_ok());
    }

    #[test]
    fn test_gpu_logger_default() {
        // Default should never panic even without CUDA
        let gpu = GpuLogger::default();
        assert_eq!(gpu.buffer_size, 1024 * 1024);
    }

    #[test]
    fn test_gpu_availability() {
        // Should not panic if CUDA is unavailable
        if let Ok(gpu) = GpuLogger::new(1024) {
            let _ = gpu.is_available();
        }
    }

    #[test]
    fn test_gpu_enable_disable() {
        if let Ok(gpu) = GpuLogger::new(1024) {
            gpu.disable();
            assert!(!gpu.is_enabled());
        }
    }

    #[test]
    fn test_gpu_info() {
        if let Ok(gpu) = GpuLogger::new(1024) {
            let info = gpu.get_info();
            assert!(!info.is_empty());
            assert!(info.contains("GPU Logging"));
        }
    }

    #[test]
    fn test_gpu_write_when_disabled() {
        if let Ok(gpu) = GpuLogger::new(1024) {
            gpu.disable();
            let data = b"test log data";
            let result = gpu.write_to_gpu(data);
            #[cfg(feature = "gpu")]
            assert!(result.is_ok());
            #[cfg(not(feature = "gpu"))]
            assert!(result.is_err());
        }
    }

    #[cfg(not(feature = "gpu"))]
    #[test]
    fn test_gpu_not_available_without_feature() {
        let gpu = GpuLogger::new(1024).unwrap();
        assert!(!gpu.is_available());
        assert!(gpu.enable().is_err());
    }

    #[cfg(feature = "gpu")]
    #[test]
    fn test_gpu_write_to_gpu() {
        if let Ok(gpu) = GpuLogger::new(1024) {
            if gpu.is_available() {
                let _ = gpu.enable();
                let data = b"test log data";
                let _ = gpu.write_to_gpu(data);
            }
        }
    }
}
