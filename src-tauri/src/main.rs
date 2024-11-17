// src-tauri/src/main.rs

use risk_normalization_lib::{
    perform_risk_normalization_basic,
    perform_risk_normalization_concurrent,
};

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            perform_risk_normalization_basic,
            perform_risk_normalization_concurrent,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}