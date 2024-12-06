use lib::feature::is_feature_enabled;
use lib::config::{ get_config, Config };

pub fn is_enabled(feature_name: String, env: Option<String>) -> bool {
    let config: &Config = get_config();
    let env: String = env.unwrap_or_else(|| std::env::var("APP_ENV").unwrap_or("dev".to_string()));
    is_feature_enabled(&feature_name, &env, &config)
}
