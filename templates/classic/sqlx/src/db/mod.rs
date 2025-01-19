use std::sync::OnceLock;

use sqlx::SqlitePool;

pub static SQLX_POOL: OnceLock<SqlitePool> = OnceLock::new();

pub fn pool() -> &'static SqlitePool {
    SQLX_POOL.get().expect("sqlx pool should set")
}
