// src/lib.rs

pub mod calculations;
pub mod utils;

pub use calculations::basic::risk_normalization_basic;
pub use calculations::concurrent::risk_normalization_concurrent;

// Re-export the RiskNormalizationResult and RiskNormalizationError structs
pub use calculations::RiskNormalizationResult;
pub use calculations::RiskNormalizationError;

use rand::rngs::StdRng;
use rand::SeedableRng;
use std::path::Path;
use std::error::Error;

// Tauri commands
// #[tauri::command] // keep this commented out for now
pub fn perform_risk_normalization_basic(
    trades: Vec<f64>,
    number_days_in_forecast: usize,
    number_trades_in_forecast: usize,
    initial_capital: f64,
    tail_percentile: f64,
    drawdown_tolerance: f64,
    number_equity_in_cdf: usize,
    number_repetitions: usize,
) -> Result<RiskNormalizationResult, String> {
    let mut rng = StdRng::from_entropy();
    risk_normalization_basic(
        &trades,
        number_days_in_forecast,
        number_trades_in_forecast,
        initial_capital,
        tail_percentile,
        drawdown_tolerance,
        number_equity_in_cdf,
        number_repetitions,
        &mut rng,
    )
    .map_err(|e| e.to_string())
}

// #[tauri::command] // keep this commented out for now
pub fn perform_risk_normalization_concurrent(
    trades: Vec<f64>,
    number_days_in_forecast: usize,
    number_trades_in_forecast: usize,
    initial_capital: f64,
    tail_percentile: f64,
    drawdown_tolerance: f64,
    number_equity_in_cdf: usize,
    number_repetitions: usize,
) -> Result<RiskNormalizationResult, String> {
    let mut rng = StdRng::from_entropy();
    risk_normalization_concurrent(
        &trades,
        number_days_in_forecast,
        number_trades_in_forecast,
        initial_capital,
        tail_percentile,
        drawdown_tolerance,
        number_equity_in_cdf,
        number_repetitions,
        &mut rng,
    )
    .map_err(|e| e.to_string())
}

// Function to read trades from a CSV file
pub fn read_trades_from_csv<P: AsRef<Path>>(filename: P) -> Result<Vec<f64>, Box<dyn Error>> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path(filename)?;
    let mut trades = Vec::new();
    for result in rdr.records() {
        let record = result?;
        for field in record.iter() {
            if let Ok(value) = field.parse::<f64>() {
                trades.push(value);
            }
        }
    }
    Ok(trades)
}