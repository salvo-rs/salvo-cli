use std::sync::OnceLock;
use rbatis::RBatis;
use rbdc_sqlite::SqliteDriver;
use anyhow::{Result, Error};
use rbatis::rbdc::db::ExecResult;

pub static RB: OnceLock<RBatis> = OnceLock::new();

pub async fn init(url: &str) -> Result<()> {
    let rb = RBatis::new();
    
    // Initialize with SQLite driver and database URL
    rb.link(SqliteDriver {}, url)
        .await
        .map_err(Error::from)?;
    
    // Store in global static
    RB.set(rb).map_err(|_| Error::msg("Failed to set global RB"))?;
    Ok(())
}

pub async fn migrate() -> Result<ExecResult> {
    let rb = RB.get().ok_or_else(|| Error::msg("Database not initialized"))?;
    
    // Create table if not exists and try to insert initial user with OR IGNORE
    Ok(rb.exec("CREATE TABLE IF NOT EXISTS users (
        id VARCHAR(255) PRIMARY KEY NOT NULL,
        username VARCHAR(255) NOT NULL UNIQUE,
        password VARCHAR(511) NOT NULL
    );
    INSERT OR IGNORE INTO users (id, username, password) 
    VALUES ('cdd0e080-5bb1-4442-b6f7-2ba60dbd0555', 'zhangsan', '$argon2id$v=19$m=19456,t=2,p=1$rcosL5pOPdA2c7i4ZuLA4Q$s0JGh78UzMmu1qZMpVUA3b8kWYLXcZhw7uBfwhYDJ4A');", vec![])
        .await
        .map_err(Error::from)?)
}

pub fn get_pool() -> Option<&'static RBatis> {
    RB.get()
}
