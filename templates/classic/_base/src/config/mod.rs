use std::sync::OnceLock;

use serde::Deserialize;

mod log;
pub use log::LogConfig;
mod db;
pub use db::DbConfig;

pub static CONFIG: OnceLock<ServerConfig> = OnceLock::new();
pub fn get() -> &'static ServerConfig {
    CONFIG.get().expect("config should be set")
}

#[derive(Deserialize, Clone, Debug)]
pub struct ServerConfig {
    #[serde(default = "default_listen_addr")]
    pub listen_addr: String,

    pub db: DbConfig,
    pub log: LogConfig,
    pub jwt: JwtConfig,
    pub tls: Option<TlsConfig>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct JwtConfig {
    pub secret: String,
    pub expires: i64,
}
#[derive(Deserialize, Clone, Debug)]
pub struct TlsConfig {
    pub cert: String,
    pub key: String,
}

pub fn default_false() -> bool {
    false
}
pub fn default_true() -> bool {
    true
}

fn default_listen_addr() -> String {
    "127.0.0.1:8008".into()
}