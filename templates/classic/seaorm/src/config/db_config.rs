use serde::{Deserialize, Serialize};

use super::default_false;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct DbConfig {
    /// Settings for the primary database. This is usually writeable, but will be read-only in
    /// some configurations.
    /// An optional follower database. Always read-only.
    #[serde(alias = "database_url", default)]
    pub url: String,

    #[serde(default = "default_min_connections")]
    pub min_connections: u32,
    #[serde(default = "default_max_connections")]
    pub max_connections: u32,

    #[serde(default = "default_idle_timeout")]
    pub idle_timeout: u32,
    #[serde(default = "default_connect_timeout")]
    pub connect_timeout: u32,

    #[serde(default = "default_false")]
    pub sqlx_logging: bool,
}

fn default_min_connections() -> u32 {
    5
}
fn default_max_connections() -> u32 {
    100
}
fn default_idle_timeout() -> u32 {
    8
}
fn default_connect_timeout() -> u32 {
    8
}
