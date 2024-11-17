// src/utils.rs

use rand::distributions::{Distribution, Uniform};
use rand::rngs::StdRng;
use statrs::statistics::Statistics;

// Function to compute mean of a slice
pub fn compute_mean(data: &[f64]) -> f64 {
    data.mean()
}

// Function to compute standard deviation of a slice
pub fn compute_std_dev(data: &[f64], mean: f64) -> f64 {
    let variance = data
        .iter()
        .map(|value| {
            let diff = value - mean;
            diff * diff
        })
        .sum::<f64>()
        / data.len() as f64;
    variance.sqrt()
}

// Function to compute statistics
pub fn compute_statistics(data: &[f64]) -> (f64, f64) {
    let mean = compute_mean(data);
    let stdev = compute_std_dev(data, mean);
    (mean, stdev)
}

// Function to calculate maximum drawdown from equity curve
pub fn calculate_drawdown(equity_curve: &[f64]) -> f64 {
    let mut peak = equity_curve[0];
    let mut max_drawdown = 0.0;
    for &equity in equity_curve.iter().skip(1) {
        if equity > peak {
            peak = equity;
        }
        let drawdown = (peak - equity) / peak;
        if drawdown > max_drawdown {
            max_drawdown = drawdown;
        }
    }
    max_drawdown
}

// Function to calculate CAGR
pub fn calculate_cagr(initial_equity: f64, final_equity: f64, years: f64) -> f64 {
    if initial_equity <= 0.0 || final_equity <= 0.0 || years <= 0.0 {
        return 0.0;
    }
    ((final_equity / initial_equity).powf(1.0 / years) - 1.0) * 100.0
}

// Function to simulate one equity sequence and calculate max drawdown
pub fn make_one_equity_sequence(
    trades: &[f64],
    fraction: f64,
    number_trades_in_forecast: usize,
    initial_capital: f64,
    rng: &mut StdRng,
) -> (Vec<f64>, f64) {
    let mut equity_curve = vec![initial_capital];
    let trade_dist = Uniform::from(0..trades.len());
    for _ in 0..number_trades_in_forecast {
        let idx = trade_dist.sample(rng);
        let trade_return = trades[idx] * fraction * equity_curve.last().unwrap();
        let new_equity = equity_curve.last().unwrap() + trade_return;
        equity_curve.push(new_equity);
    }
    let max_drawdown = calculate_drawdown(&equity_curve);
    (equity_curve, max_drawdown)
}

// Function to analyze distribution of drawdowns and compute tail risk
pub fn analyze_distribution_of_drawdown(
    trades: &[f64],
    fraction: f64,
    number_trades_in_forecast: usize,
    initial_capital: f64,
    drawdown_tolerance: f64,
    number_equity_in_cdf: usize,
    rng: &mut StdRng,
) -> f64 {
    let mut count_exceed = 0;
    for _ in 0..number_equity_in_cdf {
        let (_equity_curve, max_drawdown) = make_one_equity_sequence(
            trades,
            fraction,
            number_trades_in_forecast,
            initial_capital,
            rng,
        );
        if max_drawdown > drawdown_tolerance {
            count_exceed += 1;
        }
    }
    count_exceed as f64 / number_equity_in_cdf as f64
}