use std::fs::File;
use std::io::Read;
use std::sync::OnceLock;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Serialize)]
pub struct Sources {
    pub default: String,
    pub jira: Option<String>,
    pub bitbucket: Option<String>,
    pub github: Option<String>,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct Issue {
    pub id: String,
    pub source: Option<String>,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct Feature {
    pub environments: Vec<String>,
    pub issue: Option<Issue>,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct Config {
    pub features: HashMap<String, Feature>,
    pub sources: Sources,
}

pub static CONFIG: OnceLock<Config> = OnceLock::new();

pub fn get_config() -> &'static Config {
    CONFIG.get_or_init(|| {
        let config_file: String = std::env::var("RLSE_CONFIG").unwrap_or_else(|_| "rlse.toml".to_string());
        let mut file: File = File::open(&config_file)
            .expect(&format!("Failed to open config file: '{}'", &config_file));
        let mut contents: String = String::new();
        file.read_to_string(&mut contents).expect("Failed to read file");
        toml::from_str(&contents).expect("Invalid TOML format in config file")
    })
}