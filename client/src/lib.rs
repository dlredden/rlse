//! # Feature Flag Client
mod utils;

use wasm_bindgen::prelude::*;
use lib::config::parse_config;
use lib::feature::is_feature_enabled;

/// Check if a feature is enabled
#[wasm_bindgen]
pub fn is_enabled(feature_name: String, env: Option<String>, config: String) -> bool {
    let cfg = parse_config(&config);
    let env = env.unwrap_or_else(|| std::env::var("APP_ENV").unwrap_or("dev".to_string()));
    is_feature_enabled(&feature_name, &env, &cfg)
}
