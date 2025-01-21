use salvo::prelude::*;
use serde::{Deserialize, Serialize};
use sqlx::prelude::*;

#[derive(FromRow, Serialize, Deserialize, Extractible, Debug)]
#[salvo(extract(default_source(from = "body", parse = "json")))]
pub struct User {
    #[salvo(extract(source(from = "param")))]
    pub id: String,
    pub username: String,
    pub password: String,
}

#[derive(FromRow, Serialize, ToSchema, Debug)]
pub struct SafeUser {
    pub id: String,
    pub username: String,
}
