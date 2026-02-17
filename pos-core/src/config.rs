use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct AppConfig {
    pub pg_dsn: String,
    pub redis_url: String,
    pub kafka_brokers: String,
    pub kafka_topic: String,
    pub http_port: u16,
    pub optimistic_rpc: String,
    pub optimistic_sgc_address: String,
    pub solana_rpc: String,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, config::ConfigError> {
        let mut c = config::Config::builder()
            .add_source(config::Environment::default().separator("__"))
            .build()?;
        c.try_deserialize()
    }
}
