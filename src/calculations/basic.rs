// src/calculations/basic.rs

use rand::rngs::StdRng;
use crate::calculations::{RiskNormalizationResult, RiskNormalizationError};
use crate::utils::*;

pub fn risk_normalization_basic(
    trades: &[f64],
    number_days_in_forecast: usize,
    number_trades_in_forecast: usize,
    initial_capital: f64,
    tail_percentile: f64,
    drawdown_tolerance: f64,
    number_equity_in_cdf: usize,
    number_repetitions: usize,
    rng: &mut StdRng,
) -> Result<RiskNormalizationResult, RiskNormalizationError> {
    let desired_accuracy = 0.003;
    let mut safe_f_list = Vec::with_capacity(number_repetitions);
    let mut car25_list = Vec::with_capacity(number_repetitions);

    for _ in 0..number_repetitions {
        let mut fraction = 1.0;
        let tolerance = desired_accuracy;
        let max_iterations = 1000;
        let mut iteration = 0;

        let tail_target = tail_percentile / 100.0;

        let mut lower_bound = 0.0;
        let mut upper_bound = 10.0; // Arbitrary upper limit for fraction
        let mut _tail_risk = 0.0; // Ensure it's used

        while iteration < max_iterations {
            fraction = (lower_bound + upper_bound) / 2.0;
            _tail_risk = analyze_distribution_of_drawdown(
                trades,
                fraction,
                number_trades_in_forecast,
                initial_capital,
                drawdown_tolerance,
                number_equity_in_cdf,
                rng,
            );

            if (_tail_risk - tail_target).abs() < tolerance {
                break;
            } else if _tail_risk > tail_target {
                upper_bound = fraction;
            } else {
                lower_bound = fraction;
            }
            iteration += 1;
        }

        safe_f_list.push(fraction);

        // Simulate equity curves to collect CARs
        let mut car_list = Vec::with_capacity(number_equity_in_cdf);
        for _ in 0..number_equity_in_cdf {
            let (equity_curve, _max_drawdown) = make_one_equity_sequence(
                trades,
                fraction,
                number_trades_in_forecast,
                initial_capital,
                rng,
            );

            let years = number_days_in_forecast as f64 / 252.0;
            let cagr = calculate_cagr(
                initial_capital,
                *equity_curve.last().unwrap(),
                years,
            );
            car_list.push(cagr);
        }

        // Calculate the 25th percentile CAR (CAR25)
        car_list.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let index = ((0.25 * car_list.len() as f64).ceil() as usize).saturating_sub(1);
        let car25 = *car_list.get(index).ok_or_else(|| {
            RiskNormalizationError(format!(
                "Failed to compute CAR25 for fraction {}",
                fraction
            ))
        })?;
        car25_list.push(car25);

        // Print Compound Annual Return for this repetition
        println!("CAR25: {:.5}%", car25);
    }

    // Compute statistics for safe_f
    let (safe_f_mean, safe_f_stdev) = compute_statistics(&safe_f_list);

    // Compute statistics for CAR25
    let (car25_mean, car25_stdev) = compute_statistics(&car25_list);

    Ok(RiskNormalizationResult {
        safe_f_mean,
        safe_f_stdev,
        car25_mean,
        car25_stdev,
    })
}