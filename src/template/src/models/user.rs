use serde::Serialize;
use sqlx::FromRow;

#[derive(FromRow, Serialize, Debug)]
pub struct User {
    pub id: String,
    pub username: String,
    pub password: String,
}
