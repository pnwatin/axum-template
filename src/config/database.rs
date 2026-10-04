use std::time::Duration;

use secrecy::{ExposeSecret, SecretString};
use serde::Deserialize;
use serde_aux::field_attributes::{deserialize_bool_from_anything, deserialize_number_from_string};
use sqlx::{
    ConnectOptions, PgPool,
    postgres::{PgConnectOptions, PgPoolOptions},
};
use tracing_log::log::LevelFilter;

#[derive(Deserialize)]
pub struct DatabaseConfig {
    pub host: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub user: String,
    pub password: SecretString,
    pub name: String,
    #[serde(deserialize_with = "deserialize_bool_from_anything")]
    pub auto_migrate: bool,
}

impl DatabaseConfig {
    pub async fn connect(&self) -> Result<PgPool, sqlx::Error> {
        PgPoolOptions::new()
            .acquire_timeout(Duration::from_secs(3))
            .connect_with(self.with_database())
            .await
    }

    pub fn without_database(&self) -> PgConnectOptions {
        PgConnectOptions::new()
            .host(&self.host)
            .port(self.port)
            .username(&self.user)
            .password(self.password.expose_secret())
            .log_statements(LevelFilter::Debug)
    }

    pub fn with_database(&self) -> PgConnectOptions {
        self.without_database().database(&self.name)
    }
}
