use figment::Figment;
use serde::Deserialize;

use crate::{
    Env,
    config::{bind::BindConfig, database::DatabaseConfig},
};

#[derive(Deserialize)]
pub struct SharedConfig {
    pub env: Env,
    pub bind: BindConfig,
    pub database: DatabaseConfig,
}

impl SharedConfig {
    pub fn from_env() -> Result<Self, Box<figment::Error>> {
        let _ = dotenvy::dotenv();

        Figment::new()
            .merge(figment::providers::Env::prefixed("APP_").split("__"))
            .extract()
            .map_err(Box::new)
    }
}
