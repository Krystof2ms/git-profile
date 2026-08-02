use crate::constans::CONFIG_FILE;
use serde::{Deserialize, Serialize};
use std::{fs, io};
use toml;

#[derive(Debug, Clone, Serialize, Deserialize, Copy)]
pub enum Level {
    Session,
    Local,
    Global,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    pub id: u32,
    pub name: String,
    pub email: String,
    pub ssh_key: Option<String>,
    pub default_level: Option<Level>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Defaults {
    pub profile: Option<String>,
    pub tui: bool,
    pub level: Level,
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
            level: Level::Session,
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

    pub fn load() -> Result<Config, LoadError> {
        let file_content = fs::read_to_string(CONFIG_FILE).map_err(|e| LoadError::ReadError(e))?;
        let config: Config = toml::from_str(&file_content).map_err(|e| LoadError::ParseError(e))?;
        Ok(config)
    }

    pub fn save(&self) -> io::Result<()> {
        let file_content =
            toml::to_string(self).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        fs::write(CONFIG_FILE, file_content)?;
        Ok(())
    }
}

#[derive(Debug)]
pub enum LoadError {
    ReadError(io::Error),
    ParseError(toml::de::Error),
}

impl Profile {
    pub fn new(
        id: u32,
        name: String,
        email: String,
        ssh_key: Option<String>,
        default_level: Option<Level>,
    ) -> Self {
        Self {
            id,
            name,
            email,
            ssh_key,
            default_level,
        }
    }
}
