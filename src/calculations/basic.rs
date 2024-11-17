// src/calculations/basic.rs
pub fn compute_mean(data: &[f64]) -> f64 {
    data.iter().sum::<f64>() / data.len() as f64
}

pub fn compute_std_dev(data: &[f64], mean: f64) -> f64 {
    let variance = data.iter()
        .map(|value| {
            let diff = value - mean;
            diff * diff
        })
        .sum::<f64>() / data.len() as f64;
    variance.sqrt()
}