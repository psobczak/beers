use std::env;

use config::{Config, ConfigError, Environment, File};
use redis::{aio::ConnectionManager, Client, RedisResult};
use secrecy::{ExposeSecret, Secret};
use serde::Deserialize;
use sqlx::PgPool;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub application: ApplicationSettings,
    pub redis: RedisSettings,
    pub database: DatabaseSettings,
}

#[derive(Debug, Deserialize)]
pub struct RedisSettings {
    pub port: u16,
    pub host: String,
}

#[derive(Debug, Deserialize)]
pub struct ApplicationSettings {
    pub port: u16,
    pub host: String,
}

#[derive(Debug, Deserialize)]
pub struct DatabaseSettings {
    pub password: Secret<String>,
    pub user: String,
    pub database: String,
    pub port: u16,
    pub host: String,
}

impl DatabaseSettings {
    pub fn connect(&self) -> Result<PgPool, sqlx::Error> {
        let connection_string = format!(
            "postgres://{}:{}@{}:{}/{}",
            self.user,
            self.password.expose_secret(),
            self.host,
            self.port,
            self.database
        );

        PgPool::connect_lazy(&connection_string)
    }
}

impl ApplicationSettings {
    pub fn get_address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

impl RedisSettings {
    fn get_connection_string(&self) -> String {
        format!("redis://{}:{}", self.host, self.port)
    }

    pub async fn get_redis_connection(&self) -> RedisResult<ConnectionManager> {
        let client = Client::open(self.get_connection_string())?;
        ConnectionManager::new(client).await
    }
}

impl AppConfig {
    pub fn new() -> Result<Self, ConfigError> {
        let env = env::var("APP_ENV").unwrap_or_else(|_| "local".into());

        let config = Config::builder()
            .add_source(File::with_name("config/default.yaml"))
            .add_source(File::with_name(&format!("config/{}", env)).required(false))
            .add_source(Environment::with_prefix("app"))
            .build()?;

        config.try_deserialize()
    }
}
