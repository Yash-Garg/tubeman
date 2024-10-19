use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Config {
    pub channels: HashMap<String, Vec<String>>,
    pub settings: Option<Settings>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Settings {
    pub include_shorts: bool,
    pub show_thumbnails: bool,
    pub enable_ntfy: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            include_shorts: false,
            show_thumbnails: true,
            enable_ntfy: false,
        }
    }
}

impl Config {
    pub fn load() -> Self {
        let config_content = fs::read_to_string("config.toml").expect("Could not read config file");
        toml::from_str(&config_content).expect("Invalid TOML format")
    }
}
