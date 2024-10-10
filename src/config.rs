use crate::AppResult;

#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct Settings {
    pub colors: ColorSettings,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            colors: ColorSettings {
                primary: "#ff0000".to_string(),
                secondary: "#ffff00".to_string(),
                highlight: "#999999".to_string(),
            },
        }
    }
}

#[derive(serde::Deserialize, Debug, PartialEq)]
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

#[test]
fn configuration_is_found() {
    let config = get_configuration().unwrap();

    assert_ne!(config, Settings::default());
}
