use config::Config;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct MpesaSettings {
    pub base_url: String,
    pub consumer_key: String,
    pub consumer_secret: String,
    pub passkey: String,
    pub callback_url: String,
    pub environment: String,
    pub callback_port: u16,
    pub business_short_code: String,
    pub party_b: String,
    pub certificate_path: String,
}

#[derive(Debug, Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: u16,
    pub database_name: String,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub mpesa: MpesaSettings,
    pub database: DatabaseSettings,
}

impl Settings {
    pub fn new() -> Result<Self, config::ConfigError> {
        let settings = Config::builder()
            .add_source(config::Environment::with_prefix("MPESA"))
            .build()?;

        settings.try_deserialize()
    }
}
