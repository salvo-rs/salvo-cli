use rbatis::crud;
use serde::{Serialize, Deserialize};
use salvo::oapi::ToSchema;

#[derive(Serialize, Deserialize, Clone)]
pub struct Users {
    pub id: String,
    pub username: String,
    pub password: String,
}
crud!(Users {});

#[derive(Serialize, Deserialize, Clone, Debug, ToSchema)]
pub struct SafeUser {
    pub id: String,
    pub username: String,
}
crud!(SafeUser {});
