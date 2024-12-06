#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use lib::config::get_config;
use lib::feature::is_feature_enabled;

#[napi]
pub fn is_enabled(feature_name: String, env: Option<String>) -> bool {
    let config = get_config();
    let env = env.unwrap_or_else(|| std::env::var("APP_ENV").unwrap_or("dev".to_string()));
    is_feature_enabled(&feature_name, &env, &config)
}
