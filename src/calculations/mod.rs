// src/calculations/mod.rs

pub mod basic;
pub mod concurrent;

// Re-export functions for easier access
pub use basic::risk_normalization_basic;
pub use concurrent::risk_normalization_concurrent;

// Re-export structs and errors
// pub use RiskNormalizationResult;
// pub use RiskNormalizationError;

use serde::Serialize;
use std::fmt;
use std::error::Error;

#[derive(Debug, Serialize)]
pub struct RiskNormalizationResult {
    pub safe_f_mean: f64,
    pub safe_f_stdev: f64,
    pub car25_mean: f64,
    pub car25_stdev: f64,
}

#[derive(Debug)]
pub struct RiskNormalizationError(pub String);

impl fmt::Display for RiskNormalizationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RiskNormalizationError: {}", self.0)
    }
}

impl Error for RiskNormalizationError {}