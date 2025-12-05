use mplib::MicropubService;
use std::env::home_dir;
use std::fs;
use std::io;
use std::path::Path;
use serde::{Deserialize, Serialize};

use crate::configuration::default_behavior::DefaultBehavior;

#[derive(Serialize, Deserialize)]
pub struct AppConfig {
    pub service: MicropubService,
    pub default_behavior: DefaultBehavior,
}

impl AppConfig {
    pub fn new(service: MicropubService, default_behavior: DefaultBehavior) -> Self {
        AppConfig { service, default_behavior }
    }

    pub fn get_config_file_path() -> io::Result<String> {
        let home = home_dir().ok_or_else(|| {
            io::Error::new(io::ErrorKind::NotFound, "Could not determine home directory")
        })?;

        let config_path = home.join(".config").join("mp").join("config.toml");

        Ok(config_path.to_str().unwrap().to_string())
    }

    pub fn config_file_exists() -> bool {
        if let Ok(config_path) = AppConfig::get_config_file_path() {
            return Path::new(&config_path).exists();
        }
        false
    }

    pub fn load() -> io::Result<Self> {
        let config_path = AppConfig::get_config_file_path()?;
        if !Path::new(&config_path).exists() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "Configuration file not found. Please run 'mp configure' to create one.",
            ));
        }

        let config_content = fs::read_to_string(config_path)?;
        let app_config: AppConfig = toml::from_str(&config_content).map_err(|e| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Failed to parse configuration file: {}", e),
            )
        })?;

        Ok(app_config)
    }

    pub fn save(&self) -> io::Result<String> {
        let config_path = AppConfig::get_config_file_path()?;
        fs::create_dir_all(Path::new(&config_path).parent().unwrap())?;
        
        let config_content = toml::to_string(self).map_err(|e| {
            io::Error::new(io::ErrorKind::InvalidData, format!("Failed to serialize config: {}", e))
        })?;

        fs::write(&config_path, config_content)?;
        Ok(config_path)
    }
}
