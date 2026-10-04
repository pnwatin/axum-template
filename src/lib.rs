use serde::Deserialize;

pub mod config;
pub mod context;
pub mod middleware;
pub mod openapi;
pub mod routes;
pub mod server;
pub mod tracing;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Env {
    Test,
    Development,
    Production,
}
