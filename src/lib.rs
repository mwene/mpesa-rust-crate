use config::Config;
use serde::Deserialize;
use sqlx::PgPool;
use std::sync::Arc;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

pub mod callback;
pub mod client;
pub mod models;
pub mod settings;

#[derive(Debug, Deserialize)]
pub struct MpesaSettings {
    pub base_url: String,
    pub consumer_key: String,
    pub consumer_secret: String,
    pub passkey: String,
    pub callback_url: String,
    pub environment: String,
    pub callback_port: u16,
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

pub struct MpesaCrate {
    pub settings: Arc<Settings>,
    pub db_pool: PgPool,
}

impl MpesaCrate {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        // Initialize logging
        let subscriber = FmtSubscriber::builder()
            .with_max_level(Level::INFO)
            .finish();
        tracing::subscriber::set_global_default(subscriber)?;

        info!("Initializing MpesaCrate...");

        // Load settings
        let settings = Config::builder()
            .add_source(config::Environment::with_prefix("MPESA"))
            .build()?
            .try_deserialize()?;

        let settings = Arc::new(settings);

        // Initialize database pool
        let db_pool = PgPool::connect(&format!(
            "postgres://{}:{}@{}:{}/{}",
            settings.database.username,
            settings.database.password,
            settings.database.host,
            settings.database.port,
            settings.database.database_name,
        ))
        .await?;

        info!("Database pool initialized successfully.");

        // Create database tables
        models::create_tables(&db_pool).await?;
        info!("Database tables created successfully.");

        Ok(Self { settings, db_pool })
    }
}
