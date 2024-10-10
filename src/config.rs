use crate::AppResult;

#[derive(serde::Deserialize)]
pub struct Settings {
    pub colors: ColorSettings,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            colors: ColorSettings {
                primary: "#00ff00".to_string(),
                secondary: "#ffff00".to_string(),
                highlight: "#666666".to_string(),
            },
        }
    }
}

#[derive(serde::Deserialize)]
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
        .add_source(config::File::from_str(format!("/home/{user}/.config/crabfeed/config.yaml").as_str(), config::FileFormat::Yaml));

    match settings.build() {
        Ok(settings) => {
            match settings.try_into() {
                Ok(settings) => Ok(settings),

                Err(err) => Err(err.into()),
            }
        }
        Err(_) => {
            return Ok(Settings::default());
        },
    }

}


