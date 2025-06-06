use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub cmds: Cmds,
    pub scripts: Scripts,
}

#[derive(Debug, Deserialize)]
pub struct Cmds {
    pub install_core: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Scripts {
    pub list: Vec<String>,
}

pub fn read_config() -> Config {
    let file = fs::read_to_string("config.toml").expect("ERROR: Can't read config.toml");
    let config: Config = toml::from_str(&file).unwrap();
    return config;
}
