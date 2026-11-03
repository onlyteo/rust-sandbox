use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};

use serde::Deserialize;

use crate::error::ServerError;

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

impl ToString for ServerConfig {
    fn to_string(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

impl From<ServerConfig> for SocketAddr {
    fn from(config: ServerConfig) -> SocketAddr {
        let ip = config
            .host
            .parse::<Ipv4Addr>()
            .map_err(ServerError::Address)
            .unwrap();
        SocketAddr::V4(SocketAddrV4::new(ip, config.port))
    }
}
