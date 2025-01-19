use rinja::Template;
use salvo::oapi::extract::*;
use salvo::prelude::*;
use serde::Deserialize;
use ulid::Ulid;
use validator::Validate;

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
    let _ = sqlx::query!(
        r#"
            INSERT INTO users (id, username, password)
            VALUES ($1, $2, $3)
            "#,
        id,
        username,
        password,
    )
    .execute(conn)
    .await?;

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
    let _ = sqlx::query!(
        r#"
            UPDATE users
            SET username = $1, password = $2
            WHERE id = $3
            "#,
        username,
        password,
        user_id,
    )
    .execute(conn)
    .await?;
    json_ok(SafeUser {
        id: user_id,
        username,
    })
}

#[endpoint(tags("users"))]
pub async fn delete_user(user_id: PathParam<String>) -> EmptyResult {
    let user_id = user_id.into_inner();
    let conn = db::pool();
    sqlx::query!(
        r#"
            DELETE FROM users
            WHERE id = $1
            "#,
        user_id,
    )
    .execute(conn)
    .await?;
    empty_ok()
}

#[endpoint(tags("users"))]
pub async fn list_users() -> JsonResult<Vec<SafeUser>> {
    let conn = db::pool();
    let users = sqlx::query_as!(
        SafeUser,
        r#"
            SELECT id, username FROM users
            "#,
    )
    .fetch_all(conn)
    .await?;
    json_ok(users)
}
