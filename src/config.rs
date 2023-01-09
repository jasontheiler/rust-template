use std::net::{IpAddr, Ipv4Addr};

use figment::providers::{Env, Format, Serialized, Toml};
use figment::Figment;
use figment_file_provider_adapter::FileAdapter;
use log::Level;
use serde::{Deserialize, Serialize};

#[allow(unused)]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Config {
    pub log: String,
    pub host: IpAddr,
    pub port: u16,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            log: String::from(Level::Info.as_str()),
            host: IpAddr::V4(Ipv4Addr::LOCALHOST),
            port: 8080,
        }
    }
}

impl Config {
    pub fn new() -> Result<Self, figment::Error> {
        Figment::from(Serialized::defaults(Self::default()))
            .merge(Toml::file("./config.toml"))
            .merge(FileAdapter::wrap(Env::raw()))
            .extract()
    }
}
