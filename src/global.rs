use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub cmds: Cmds,
}

#[derive(Debug, Deserialize)]
pub struct Cmds {
    pub shell_cmd: Vec<String>,
    pub install_core: String,
}

pub fn read_config() -> Config {
    let file = fs::read_to_string("config.toml")
        .expect("ERROR: Can't read config.toml");
    let config: Config = toml::from_str(&file).unwrap();
    return config;
}
