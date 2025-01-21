use mongodb::bson::oid::ObjectId;
use salvo::oapi::ToSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    _id: Option<ObjectId>,
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct SafeUser {
    pub id: String,
    pub username: String,
}
