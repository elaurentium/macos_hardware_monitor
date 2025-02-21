use serde::Deserialize;
use std::fs;

#[derive(Debug, Clone, Deserialize)]
pub struct Settings {
    pub update_interval_secs: u64,
    pub show_cpu: bool,
    pub show_memory: bool,
    pub show_disk: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub settings: Settings,
}

impl Config {
    pub fn load(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let config = toml::from_str(&content)?;
        Ok(config)
    }

    pub fn default() -> Self {
        Config {
            settings: Settings {
                update_interval_secs: 2,
                show_cpu: true,
                show_memory: true,
                show_disk: true,
            },
        }
    }
}