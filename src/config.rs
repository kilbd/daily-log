use std::fs;

use color_eyre::eyre::{eyre, Result, WrapErr};
use directories::ProjectDirs;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub base_dir: String,
    pub editor: Option<String>,
}

pub fn get_config() -> Result<Config> {
    if let Some(project_dir) = ProjectDirs::from("com", "kilb", "daily-log") {
        let config_data = fs::read_to_string(project_dir.config_dir().join("config.toml"))
            .wrap_err("Unable to read configuration file.")?;
        toml::from_str::<Config>(&config_data).wrap_err("Failed to parse TOML config.")
    } else {
        Err(eyre!("Unable to find config directory"))
    }
}
