// src/main.rs

use rand::rngs::StdRng;
use rand::SeedableRng;
use risk_normalization::{read_trades_from_csv, risk_normalization};
use std::error::Error;
use std::process;

fn main() -> Result<(), Box<dyn Error>> {
    // Define the path to the CSV file
    let base_path_to_trades = "./data/";
    let file_name = "generated_normal_trades.csv";
    let path_to_trades = format!("{}{}", base_path_to_trades, file_name);

    println!("\nThe data file being processed is: {}", path_to_trades);

    // Read trades from CSV
    let trades = match read_trades_from_csv(&path_to_trades) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Error reading trades: {}", e);
            process::exit(1);
        }
    };

    if trades.is_empty() {
        eprintln!("No trades data found.");
        process::exit(1);
    }

    println!(
        "There are {} marked-to-market daily trades in the file",
        trades.len()
    );
    println!("Here are the first 10 trades:");
    for trade in trades.iter().take(10) {
        println!("{}", trade);
    }

    let number_of_years_in_csv = 28.0;
    let average_trades_per_year = trades.len() as f64 / number_of_years_in_csv;
    let years_to_forecast = 2.0;

    // Calculate number of days and trades in forecast period
    let number_days_in_forecast = (years_to_forecast * 252.0) as usize; // Assuming 252 trading days per year
    let number_trades_in_forecast = (average_trades_per_year * years_to_forecast) as usize;

    let initial_capital = 100000.0;
    let tail_percentile = 5.0;
    let drawdown_tolerance = 0.10;
    let number_equity_in_cdf = 10000;
    let number_repetitions = 5;

    // Define the seed option
    let seed: Option<u64> = Some(42); // Some(seed) for fixed seed, None for random seed
    // let seed: Option<u64> = None; // Some(seed) for fixed seed, None for random seed

    // Initialize RNG based on the seed
    let mut rng = match seed {
        Some(seed_value) => StdRng::seed_from_u64(seed_value),
        None => StdRng::from_entropy(),
    };

    // Call risk_normalization function
    let result = match risk_normalization(
        &trades,
        number_days_in_forecast,
        number_trades_in_forecast,
        initial_capital,
        tail_percentile,
        drawdown_tolerance,
        number_equity_in_cdf,
        number_repetitions,
        &mut rng,
    ) {
        Ok(res) => res,
        Err(e) => {
            eprintln!("Error in risk_normalization: {}", e);
            process::exit(1);
        }
    };

    // Print results with high precision
    println!("Risk Normalization Results:");
    println!("CAR25 Mean:   {:.5}%", result.car25_mean);
    println!("CAR25 Std Dev:  {:.5}", result.car25_stdev);
    println!("Safe-F Mean:  {:.5}", result.safe_f_mean);
    println!("Safe-F Std Dev: {:.5}", result.safe_f_stdev);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_mean() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        assert_eq!(compute_mean(&data), 3.0);
    }

    #[test]
    fn test_compute_std_dev() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let mean = 3.0;
        let expected_std_dev = 1.414213; // Adjust as needed
        let calculated_std_dev = compute_std_dev(&data, mean);
        assert!((calculated_std_dev - expected_std_dev).abs() < 1e-6);
    }

    #[test]
    fn test_calculate_drawdown() {
        let equity_curve = vec![100.0, 110.0, 105.0, 115.0, 90.0];
        assert!((calculate_drawdown(&equity_curve) - 0.2173913).abs() < 1e-5);
    }

    #[test]
    fn test_calculate_cagr() {
        let initial = 100.0;
        let final_val = 200.0; // Renamed from `final` to `final_val`
        let years = 2.0;
        let expected_cagr = 41.421356;
        let calculated_cagr = calculate_cagr(initial, final_val, years);
        assert!(
            (calculated_cagr - expected_cagr).abs() < 1e-5,
            "Calculated CAGR: {:.6}, Expected CAGR: {:.6}",
            calculated_cagr,
            expected_cagr
        );
    }
}
