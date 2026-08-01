use crate::constans::CONFIG_FILE;
use serde::{Deserialize, Serialize};
use std::{fs, io};
use toml;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    pub id: String,
    pub name: String,
    pub email: String,
    pub ssh_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Defaults {
    pub profile: Option<String>,
    pub tui: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub defaults: Defaults,
    pub profiles: Vec<Profile>,
}

impl Defaults {
    fn default() -> Self {
        Self {
            profile: None,
            tui: false,
        }
    }
}

impl Config {
    pub fn default() -> Self {
        Self {
            defaults: Defaults::default(),
            profiles: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub enum LoadError {
    ReadError(io::Error),
    ParseError(toml::de::Error),
}

pub fn load_config() -> Result<Config, LoadError> {
    let file_content = fs::read_to_string(CONFIG_FILE).map_err(|e| LoadError::ReadError(e))?;
    let config: Config = toml::from_str(&file_content).map_err(|e| LoadError::ParseError(e))?;
    Ok(config)
}

pub fn save_config(config: &Config) -> io::Result<()> {
    let file_content =
        toml::to_string(config).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    fs::write(CONFIG_FILE, file_content)?;
    Ok(())
}
