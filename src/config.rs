use std::fs::create_dir_all;

use crate::AppResult;

#[derive(serde::Deserialize, Debug, PartialEq, Clone)]
pub struct Settings {
    pub colors: ColorSettings,
    pub database_url: String,
}

impl Default for Settings {
    fn default() -> Self {
        let user = std::env::var("USER").unwrap();
        create_dir_all(format!("/home/{user}/.local/share/crabfeed"))
            .expect("Failed to create directory");

        Settings {
            colors: ColorSettings {
                primary: "#ff0000".to_string(),
                secondary: "#ffff00".to_string(),
                highlight: "#999999".to_string(),
            },
            database_url: format!("sqlite:///home/{user}/.local/share/crabfeed/crabfeed.db"),
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
    let user = std::env::var("USER")?;

    let settings = config::Config::builder()
        .add_source(config::File::with_name(
            format!("/home/{user}/.config/crabfeed/config.yaml").as_str(),
        ))
        .build()?;

    Ok(settings.try_into()?)
}
