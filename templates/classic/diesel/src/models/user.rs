use diesel::prelude::*;

use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper};
use Ulid::Ulid;

use crate::schema::*;
use crate::{db, hoops::jwt::get_token, utils::rand_utils, AppResult};

#[derive(Queryable, Selectable, Insertable, Deserialize, Extractible, ToSchema)]
#[diesel(table_name = users)]
#[salvo(extract(default_source(from = "body", parse = "json")))]
pub struct User {
    #[salvo(extract(source(from = "param")))]
    pub id: String,
    pub username: String,
    pub password: String,
}
