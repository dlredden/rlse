use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

#[derive(Deserialize, Debug, Serialize)]
struct Sources {
    default: String,
    jira: Option<String>,
    bitbucket: Option<String>,
    github: Option<String>,
}

#[derive(Deserialize, Debug, Serialize)]
struct Issue {
    id: String,
    source: Option<String>,
}

#[derive(Deserialize, Debug, Serialize)]
struct Feature {
    environments: Vec<String>,
    issue: Option<Issue>,
}

#[derive(Deserialize, Debug, Serialize)]
struct Config {
    features: HashMap<String, Feature>,
    sources: Sources,
}

fn get_config() -> Config {
    let config_file = std::env::var("CF3_CONFIG").unwrap_or("cf3.toml".to_string());
    let mut file =
        File::open(&config_file).expect(&format!("Failed to open config file: '{}'", &config_file));
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read file");
    // let main_table = contents.parse::<toml::Table>().unwrap();
    let config: Config = toml::from_str(&contents).unwrap();
    config
}

// Is a feature enabled in the given environment?
pub fn is_enabled(feature_name: &str, env: &str) -> bool {
    let config = get_config();
    println!("CF3_CONFIG: {:?}", config);

    let feature = config.features.get(feature_name);
    if let Some(f1) = feature {
        return f1.environments.contains(&env.to_string());
    }
    false
}
