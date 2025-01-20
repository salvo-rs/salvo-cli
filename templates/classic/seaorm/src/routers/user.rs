use rinja::Template;
use salvo::oapi::extract::*;
use salvo::prelude::*;
use sea_orm::{ActiveModelTrait, EntityTrait, Set};
use serde::Deserialize;
use ulid::Ulid;
use validator::Validate;

use crate::entities::{prelude::Users, users};
use crate::models::SafeUser;
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
pub async fn create_user(idata: JsonBody<CreateInData>) -> JsonResult<SafeUser> {
    let CreateInData { username, password } = idata.into_inner();
    let id = Ulid::new().to_string();
    let password = utils::hash_password(&password).await?;
    let conn = db::pool();
    let model = users::ActiveModel {
        id: Set(id.clone()),
        username: Set(username.clone()),
        password: Set(password.clone()),
    };
    Users::insert(model).exec(conn).await?;

    json_ok(SafeUser { id, username })
}

#[derive(Deserialize, Debug, Validate, ToSchema)]
struct UpdateInData {
    #[validate(length(min = 5, message = "username length must be greater than 5"))]
    username: String,
    #[validate(length(min = 6, message = "password length must be greater than 5"))]
    password: String,
}
#[endpoint(tags("users"), parameters(("id", description = "user id")))]
pub async fn update_user(
    user_id: PathParam<String>,
    idata: JsonBody<UpdateInData>,
) -> JsonResult<SafeUser> {
    let user_id = user_id.into_inner();
    let UpdateInData { username, password } = idata.into_inner();
    let conn = db::pool();

    let Some(user) = Users::find_by_id(user_id).one(conn).await? else {
        return Err(anyhow::anyhow!("User does not exist.").into());
    };
    let mut user: users::ActiveModel = user.into();
    user.username = Set(username.to_owned());
    user.password = Set(utils::hash_password(&password).await?);

    let user: users::Model = user.update(conn).await?;
    json_ok(SafeUser {
        id: user.id,
        username: user.username,
    })
}

#[endpoint(tags("users"))]
pub async fn delete_user(user_id: PathParam<String>) -> EmptyResult {
    let user_id = user_id.into_inner();
    let conn = db::pool();
    Users::delete_by_id(user_id).exec(conn).await?;
    empty_ok()
}

#[endpoint(tags("users"))]
pub async fn list_users() -> JsonResult<Vec<SafeUser>> {
    let conn = db::pool();
    let users = Users::find()
        .all(conn)
        .await?
        .into_iter()
        .map(|user| SafeUser {
            id: user.id,
            username: user.username,
        })
        .collect::<Vec<_>>();
    json_ok(users)
}
