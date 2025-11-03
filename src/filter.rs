//! Log record filtering
//!
//! Provides filtering capabilities for log records based on level, module, and function.
//! Filters are applied before records are written to sinks.

use crate::level::Level;
use crate::record::LogRecord;

/// Filter for log records based on level, module, and function.
/// 
/// Filters determine which log records should be processed by a sink.
/// Multiple filter criteria can be combined (all must match).
pub struct Filter {
    /// Minimum log level to accept (records below this level are filtered out)
    min_level: Option<Level>,
    /// Module name to match (exact match required)
    module: Option<String>,
    /// Function name to match (exact match required)
    function: Option<String>,
}

impl Filter {
    /// Creates a new filter with the specified criteria.
    /// 
    /// # Arguments
    /// 
    /// * `min_level` - Minimum log level to accept
    /// * `module` - Module name to match (None accepts all)
    /// * `function` - Function name to match (None accepts all)
    pub fn new(
        min_level: Option<Level>,
        module: Option<String>,
        function: Option<String>,
    ) -> Self {
        Self {
            min_level,
            module,
            function,
        }
    }

    /// Checks if a log record matches all filter criteria.
    /// 
    /// # Arguments
    /// 
    /// * `record` - The log record to check
    /// 
    /// # Returns
    /// 
    /// `true` if the record matches all criteria, `false` otherwise
    pub fn matches(&self, record: &LogRecord) -> bool {
        if let Some(min_level) = self.min_level {
            if record.level < min_level {
                return false;
            }
        }

        if let Some(ref module_filter) = self.module {
            if let Some(ref record_module) = record.module {
                if record_module != module_filter {
                    return false;
                }
            } else {
                return false;
            }
        }

        if let Some(ref function_filter) = self.function {
            if let Some(ref record_function) = record.function {
                if record_function != function_filter {
                    return false;
                }
            } else {
                return false;
            }
        }

        true
    }
}
