use std::sync::OnceLock;

use figment::Figment;
use figment::providers::{Env, Format, Toml};
use serde::Deserialize;

mod log_config;
pub use log_config::LogConfig;
mod db_config;
pub use db_config::DbConfig;

pub static CONFIG: OnceLock<ServerConfig> = OnceLock::new();

pub fn init() {
    let raw_config = Figment::new()
        .merge(Toml::file(
            Env::var("APP_CONFIG").as_deref().unwrap_or("config.toml"),
        ))
        .merge(Env::prefixed("APP_").global());

    let mut config = match raw_config.extract::<ServerConfig>() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("It looks like your config is invalid. The following error occurred: {e}");
            std::process::exit(1);
        }
    };
    if config.db.url.is_empty() {
        config.db.url = std::env::var("DATABASE_URL").unwrap_or_default();
    }
    if config.db.url.is_empty() {
        eprintln!("DATABASE_URL is not set");
        std::process::exit(1);
    }
    crate::config::CONFIG
        .set(config)
        .expect("config should be set");
}
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
    pub expiry: i64,
}
#[derive(Deserialize, Clone, Debug)]
pub struct TlsConfig {
    pub cert: String,
    pub key: String,
}

#[allow(dead_code)]
pub fn default_false() -> bool {
    false
}
#[allow(dead_code)]
pub fn default_true() -> bool {
    true
}

fn default_listen_addr() -> String {
    "127.0.0.1:8008".into()
}