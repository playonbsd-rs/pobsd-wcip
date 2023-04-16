use directories::{self, BaseDirs};
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Deserialize, Default, Debug, Eq, PartialEq)]
pub struct Config {
    pub download_path: Option<PathBuf>,
    pub steam_id: Option<String>,
    pub steam_key: Option<String>,
    pub steam_password: Option<String>,
}

fn get_config_file_path() -> Option<PathBuf> {
    match BaseDirs::new() {
        Some(base_dir) => {
            let mut path = PathBuf::from(base_dir.config_dir());
            path.push("wcip/config.toml");
            Some(path)
        }
        None => None,
    }
}

fn get_config_content() -> Result<String, std::io::Error> {
    match get_config_file_path() {
        Some(file) => {
            let content = std::fs::read_to_string(file)?;
            Ok(content)
        }
        None => Ok(String::from("")),
    }
}

fn content_to_config(content: String) -> Result<Config, Box<dyn std::error::Error>> {
    let config: Config = toml::from_str(&content)?;
    Ok(config)
}

pub fn get_config() -> Result<Config, Box<dyn std::error::Error>> {
    let content = get_config_content()?;
    let config = content_to_config(content)?;
    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::*;
    fn get_test_config() -> String {
        String::from(
            "download_path = \"test\"\n
        steam_id = \"123\"\n
        steam_key = \"456\"",
        )
    }
    #[test]
    fn test_content_to_config() {
        let content = get_test_config();
        let exp = Config {
            steam_id: Some("123".to_string()),
            steam_key: Some("456".to_string()),
            download_path: Some(PathBuf::from("test")),
        };
        let conf = content_to_config(content).unwrap();
        assert_eq!(conf, exp);
    }
}
