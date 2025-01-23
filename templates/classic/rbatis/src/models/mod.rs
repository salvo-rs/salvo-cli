use rbatis::crud;
use salvo::oapi::ToSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    pub id: String,
    pub username: String,
    pub password: String,
}
crud!(User {},"users");

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct SafeUser {
    pub id: String,
    pub username: String,
}
crud!(SafeUser {});
