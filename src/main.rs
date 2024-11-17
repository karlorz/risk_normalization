// src/main.rs

use risk_normalization_lib::{
    read_trades_from_csv, risk_normalization_basic, risk_normalization_concurrent,
};
use rand::rngs::StdRng;
use rand::SeedableRng;
use std::env;
use std::error::Error;
use std::process;

fn main() -> Result<(), Box<dyn Error>> {
    // Parse command-line arguments
    let args: Vec<String> = env::args().collect();
    let mode = if args.len() > 1 {
        args[1].as_str()
    } else {
        "basic" // Default to 'basic' mode
    };

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
    let number_equity_in_cdf = 100000;
    let number_repetitions = 5;

    // Define the seed option
    let seed: Option<u64> = Some(42); // Some(seed) for fixed seed, None for random seed

    // Initialize RNG based on the seed
    let mut rng = match seed {
        Some(seed_value) => StdRng::seed_from_u64(seed_value),
        None => StdRng::from_entropy(),
    };

    // Call the appropriate risk_normalization function
    let result = match mode {
        "basic" => {
            println!("\nRunning in BASIC mode.");
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
        }
        "concurrent" => {
            println!("\nRunning in CONCURRENT mode.");
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
        }
        _ => {
            eprintln!("Invalid mode specified. Use 'basic' or 'concurrent'.");
            process::exit(1);
        }
    };

    // Handle the result
    let result = match result {
        Ok(res) => res,
        Err(e) => {
            eprintln!("Error in risk_normalization: {}", e);
            process::exit(1);
        }
    };

    // Print results with high precision
    println!("\nRisk Normalization Results:");
    println!("CAR25 Mean:    {:.5}%", result.car25_mean);
    println!("CAR25 Std Dev: {:.5}", result.car25_stdev);
    println!("Safe-F Mean:   {:.5}", result.safe_f_mean);
    println!("Safe-F Std Dev: {:.5}", result.safe_f_stdev);

    Ok(())
}