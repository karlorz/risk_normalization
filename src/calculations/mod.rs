// src/calculations/mod.rs

use serde::Serialize;
use std::fmt;
use std::error::Error;

pub mod basic;
pub mod concurrent;

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