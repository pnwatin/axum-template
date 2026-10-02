use serde_aux::field_attributes::deserialize_number_from_string;
use std::net::{IpAddr, SocketAddr};

use serde::Deserialize;

#[derive(Deserialize)]
pub struct BindConfig {
    pub host: IpAddr,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
}

impl BindConfig {
    pub fn address(&self) -> SocketAddr {
        SocketAddr::new(self.host, self.port)
    }
}
