use diesel::prelude::*;
use salvo::prelude::*;
use serde::{Serialize, Deserialize};

use crate::schema::*;

#[derive(Queryable, Selectable, Insertable, Deserialize, Extractible, ToSchema)]
#[diesel(table_name = users)]
#[salvo(extract(default_source(from = "body", parse = "json")))]
pub struct User {
    #[salvo(extract(source(from = "param")))]
    pub id: String,
    pub username: String,
    pub password: String,
}

#[derive(Queryable, Selectable, Serialize, ToSchema, Debug)]
#[diesel(table_name = users)]
pub struct SafeUser {
    pub id: String,
    pub username: String,
}