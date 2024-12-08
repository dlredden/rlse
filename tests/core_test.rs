#[cfg(test)]
mod tests {
  use lib::config::{get_config, Config};
  use lib::feature::is_feature_enabled;

    #[test]
    fn test_is_feature_enabled() {
        let config: &Config = get_config();
        println!("{:?}", config);

        assert!(is_feature_enabled("testFeature1", "dev", &config));
        assert!(!is_feature_enabled("testFeature", "staging", &config));
    }
}