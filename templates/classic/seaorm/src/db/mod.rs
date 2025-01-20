use std::sync::OnceLock;
use std::time::Duration;

use sea_orm::entity::prelude::DatabaseConnection;
use sea_orm::{ConnectOptions, Database};

use crate::config::DbConfig;

pub static SEAORM_POOL: OnceLock<DatabaseConnection> = OnceLock::new();

pub async fn init(config: &DbConfig) {
    let mut opt = ConnectOptions::new(config.url.to_owned());
    opt.max_connections(config.max_connections)
        .min_connections(config.min_connections)
        .connect_timeout(Duration::from_secs(config.connect_timeout as u64))
        .idle_timeout(Duration::from_secs(config.idle_timeout as u64))
        .sqlx_logging(config.sqlx_logging);

    let pool = Database::connect(opt)
        .await
        .expect("db connection should connect");
    SEAORM_POOL.set(pool).expect("seaorm pool should be set");
}

pub fn pool() -> &'static DatabaseConnection {
    SEAORM_POOL.get().expect("seaorm pool should set")
}
