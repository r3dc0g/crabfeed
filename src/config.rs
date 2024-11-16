use std::fs::create_dir_all;

use directories::BaseDirs;

use crate::AppResult;

#[derive(serde::Deserialize, Debug, PartialEq, Clone)]
pub struct Settings {
    pub colors: ColorSettings,
    pub database_url: String,
}

impl Default for Settings {
    fn default() -> Self {
        let dir = BaseDirs::new().expect("Failed to get base directories");
        let dir_str = dir
            .data_local_dir()
            .to_str()
            .expect("Failed to convert path to string");

        create_dir_all(format!("{}/crabfeed", dir_str)).expect("Failed to create directory");

        Settings {
            colors: ColorSettings {
                primary: "#ff0000".to_string(),
                secondary: "#ffff00".to_string(),
                highlight: "#999999".to_string(),
            },
            database_url: format!("sqlite:/{}/crabfeed/crabfeed.db", dir_str),
        }
    }
}

#[derive(serde::Deserialize, Debug, PartialEq, Clone)]
pub struct ColorSettings {
    pub primary: String,
    pub secondary: String,
    pub highlight: String,
}

impl TryFrom<config::Config> for Settings {
    type Error = config::ConfigError;

    fn try_from(config: config::Config) -> Result<Self, Self::Error> {
        config.try_deserialize()
    }
}

pub fn get_configuration() -> AppResult<Settings> {
    let dir = BaseDirs::new().expect("Failed to get base directories");
    let dir_str = dir
        .config_dir()
        .to_str()
        .expect("Failed to convert path to string");

    create_dir_all(format!("{}/crabfeed", dir_str)).expect("Failed to create directory");

    let settings = config::Config::builder()
        .add_source(config::File::with_name(
            format!("{}/crabfeed/config.yaml", dir_str).as_str(),
        ))
        .build()?;

    return Ok(settings.try_into()?);
}
