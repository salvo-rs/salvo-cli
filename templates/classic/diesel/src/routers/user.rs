use askama::Template;
use diesel::prelude::*;
use salvo::oapi::extract::*;
use salvo::prelude::*;
use serde::{Deserialize, Serialize};
use ulid::Ulid;
use validator::Validate;

use crate::hoops::jwt::decode_token;
use crate::models::User;
use crate::schema::*;
use crate::{db, empty_ok, json_ok, utils, AppResult, EmptyResult, JsonResult};

#[derive(Template)]
#[template(path = "user_list_page.html")]
pub struct UserListPageTemplate {}

#[derive(Template)]
#[template(path = "user_list_frag.html")]
pub struct UserListFragTemplate {}

#[handler]
pub async fn list_page(req: &mut Request, res: &mut Response) -> AppResult<()> {
    let is_fragment = req.headers().get("X-Fragment-Header");
    match is_fragment {
        Some(_) => {
            let hello_tmpl = UserListFragTemplate {};
            res.render(Text::Html(hello_tmpl.render().unwrap()));
        }
        None => {
            let hello_tmpl = UserListPageTemplate {};
            res.render(Text::Html(hello_tmpl.render().unwrap()));
        }
    }
    Ok(())
}

#[derive(Deserialize, Debug, Validate, ToSchema, Default)]
pub struct CreateInData {
    #[validate(length(min = 5, message = "username length must be greater than 5"))]
    pub username: String,
    #[validate(length(min = 6, message = "password length must be greater than 5"))]
    pub password: String,
}
#[endpoint(tags("users"))]
pub async fn create_user(idata: JsonBody<CreateInData>) -> JsonResult<User> {
    let CreateInData { username, password } = idata.into_inner();
    let mut conn = &mut db::connect()?;
    let user = User {
        id: Ulid::new().to_string(),
        username,
        password: utils::hash_password(password).await?,
    };
    let user = diesel::insert_into(users::table)
        .values(&user)
        .get_result::<User>(conn)?;
    json_ok(user)
}

#[derive(Deserialize, Debug, Validate, ToSchema, Default)]
pub struct UpdateInData {
    #[validate(length(min = 5, message = "username length must be greater than 5"))]
    pub username: String,
    #[validate(length(min = 6, message = "password length must be greater than 5"))]
    pub password: String,
}
#[endpoint(tags("users"), parameters(("id", description = "user id")))]
pub async fn update_user(
    user_id: PathParam<String>,
    idata: JsonBody<UpdateInData>,
) -> JsonResult<User> {
    let user_id = user_id.into_inner();
    let idata = idata.into_inner();
    let conn = &mut db::connect()?;
    let user = diesel::update(users::table.find(user_id))
        .set((
            users::username.eq(&idata.username),
            users::password.eq(utils::hash_password(idata.password).await?),
        ))
        .get_result::<User>(conn)?;
    json_ok(user)
}

#[endpoint(tags("users"))]
pub async fn delete_user(user_id: PathParam<String>) -> EmptyResult {
    let conn = &mut db::connect()?;
    diesel::delete(users::table.find(user_id.into_inner())).execute(conn)?;
    empty_ok()
}

#[endpoint(tags("users"))]
pub async fn list_users() -> JsonResult<Vec<User>> {
    let conn = &mut db::connect()?;
    let users = users::table.load::<User>(conn)?;
    json_ok(users)
}
