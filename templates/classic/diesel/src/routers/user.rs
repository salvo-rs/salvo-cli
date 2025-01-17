use serde::{Deserialize, Serialize};

use askama::Template;
use salvo::prelude::*;
use validator::Validate;

use crate::hoops::jwt::decode_token;
use crate::{empty_ok, json_ok, AppResult, JsonResult};

#[derive(Template)]
#[template(path = "user_list_page.html")]
pub struct UserListPageTemplate {}

#[derive(Template)]
#[template(path = "user_list.html")]
pub struct UserListTemplate {}

#[handler]
pub async fn list_page(req: &mut Request, res: &mut Response) -> AppResult<()> {
    let is_fragment = req.headers().get("X-Fragment-Header");
    match is_fragment {
        Some(_) => {
            let hello_tmpl = UserListTemplate {};
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
pub async fn create_user(new_user: JsonBody<NewUser>) -> JsonResult<User> {
    let mut conn = &mut db::connect()?;
    let user = User {
        id: Ulid::new().to_string(),
        username: req.username.clone(),
        password: rand_utils::hash_password(req.password).await?,
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
    user_id: PathParam<i64>,
    pdata: JsonBody<UpdateInData>,
) -> JsonResult<User> {
    let pdata = pdata.into_inner();
    let conn = &mut db::connect()?;
    let user = diesel::update(users::table.find(user_id.into_inner()))
        .set((
            username.eq(&pdata.username),
            password.eq(rand_utils::hash_password(&pdata.password).await?),
        ))
        .get_result::<User>(conn)?;
    json_ok(user)
}

#[endpoint(tags("users"))]
pub async fn delete_user(user_id: PathParam<String>) -> EmptyResult<()> {
    let conn = &mut db::connect()?;
    diesel::delete(users::table.find(user_id.into_inner())).execute(conn)?;
    empty_ok()
}

#[endpoint(tags("users"))]
pub async fn list_users() -> JsonResult<Vec<User>> {
    let conn = &mut db::connect()?;
    let users = users::table.load::<User>(conn)?;
    Ok(users)
}
