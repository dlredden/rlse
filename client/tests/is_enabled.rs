extern crate wasm_bindgen_test;

#[allow(dead_code)]
#[cfg(test)]
mod server_tests {
    use client::is_enabled;
    use wasm_bindgen_test::wasm_bindgen_test;

    #[test]
    fn rust_test() {
      let config: String = r#"
        [features]
        testFeature1 = { environments = ['dev', 'test', 'prod']}
        testFeature2 = { environments = ['test']}
        testFeature3 = { environments = ['uat', 'test']}
      "#.to_string();
      assert!(is_enabled("testFeature1".to_string(), Some("dev".to_string()), config.clone()));
      assert!(!is_enabled("testFeature".to_string(), Some("staging".to_string()), config.clone()));
    }

    #[wasm_bindgen_test]
    fn node_test() {
     let config: String = r#"
        [features]
        testFeature1 = { environments = ['dev', 'test', 'prod']}
        testFeature2 = { environments = ['test']}
        testFeature3 = { environments = ['uat', 'test']}
      "#.to_string(); 
      assert!(is_enabled("testFeature1".to_string(), Some("dev".to_string()), config.clone()));
      assert!(!is_enabled("testFeature".to_string(), Some("staging".to_string()), config.clone()));
    }
}

#[allow(dead_code)]
#[cfg(test)]
mod browser_tests {
    use client::is_enabled;
    use wasm_bindgen_test::{wasm_bindgen_test_configure, wasm_bindgen_test};

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn web_test() {
      let config: String = r#"
        [features]
        testFeature1 = { environments = ['dev', 'test', 'prod']}
        testFeature2 = { environments = ['test']}
        testFeature3 = { environments = ['uat', 'test']}
      "#.to_string();

      assert!(!is_enabled("testFeature1".to_string(), Some("dev".to_string()), config.clone()));
      assert!(!is_enabled("testFeature".to_string(), Some("staging".to_string()), config.clone()));
    }
}