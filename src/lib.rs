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

fn main() {
    let config = std::env::var("CF3_CONFIG").unwrap_or("cf3.toml".to_string());
    let env = std::env::var("APP_ENV").unwrap_or("cf3.toml".to_string());
    // println!("CF3_CONFIG: {:?}", config);

    let mut file =
        File::open(&config).expect(&format!("Failed to open config file: '{}'", &config));
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read file");
    // let main_table = contents.parse::<toml::Table>().unwrap();
    let main_table: Config = toml::from_str(&contents).unwrap();
    // println!("{:?}", main_table);

    let f_name = "testFeature2";
    let feature = is_enabled(f_name, &env, &main_table);
    println!("Feature {} found: {}", f_name, feature);
}

// Is a feature enabled in the given environment?
fn is_enabled(feature_name: &str, env: &str, main_table: &Config) -> bool {
    let feature = main_table.features.get(feature_name);
    if let Some(f1) = feature {
        return f1.environments.contains(&env.to_string());
    }
    false
}
