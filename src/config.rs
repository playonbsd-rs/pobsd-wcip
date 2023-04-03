use directories::{self, BaseDirs};
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Deserialize, Default, Debug)]
pub struct Config {
    pub download_path: Option<String>,
    pub steam_id: Option<String>,
    pub steam_key: Option<String>,
}

fn get_config_file_path() -> Option<PathBuf> {
    if let Some(base_dir) = BaseDirs::new() {
        let mut path = PathBuf::from(base_dir.config_dir());
        path.push("wcip/config.toml");
        Some(path)
    } else {
        None
    }
}

fn get_config_content() -> Result<String, std::io::Error> {
    if let Some(file) = get_config_file_path() {
        let content = std::fs::read_to_string(file)?;
        Ok(content)
    } else {
        Ok(String::from(""))
    }
}

pub fn get_config() -> Config {
    if let Ok(content) = get_config_content() {
        let config: Config = toml::from_str(&content).unwrap();
        return config;
    } else {
        eprintln!("Could not open config file, using default values");
    }
    Config::default()
}
