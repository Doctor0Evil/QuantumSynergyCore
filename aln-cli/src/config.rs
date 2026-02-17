use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct CliConfig {
    pub pos_endpoint: String,
}

impl CliConfig {
    pub fn from_env() -> Result<Self, config::ConfigError> {
        let mut c = config::Config::builder()
            .add_source(config::Environment::default().separator("__"))
            .build()?;
        c.try_deserialize()
    }
}
