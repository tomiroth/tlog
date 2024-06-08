use serde::Deserialize;
use std::fs::{self, read, File};
use std::path::Path;
use toml;

#[derive(Deserialize)]
pub struct Config {
    pub editor: Option<String>,
}

impl Config {
    pub fn new(config_file: &str) -> Self {
        let path = Path::new(config_file);

        if path.is_file() {
            let config: String = String::from_utf8_lossy(&read(path).unwrap())
                .parse()
                .unwrap();

            let config: Config = toml::from_str(&config).unwrap();
            return config;
        }

        Config { editor: None }
    }
}
