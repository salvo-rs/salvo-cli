use rinja::Template;
use salvo::oapi::extract::*;
use salvo::prelude::*;
use serde::{Deserialize, Serialize};
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

#[derive(Debug, Deserialize, Validate, Extractible, ToSchema)]
#[salvo(extract(default_source(from = "query")))]
pub struct UserListQuery {
    pub username: Option<String>,
    #[serde(default = "default_page")]
    pub current_page: i64,
    #[serde(default = "default_page_size")]
    pub page_size: i64,
}

fn default_page() -> i64 { 1 }
fn default_page_size() -> i64 { 10 }

#[derive(Debug, Serialize, ToSchema)]
pub struct UserListResponse {
    pub data: Vec<SafeUser>,
    pub total: i64,
    pub current_page: i64,
    pub page_size: i64,
}

#[endpoint(tags("users"))]
pub async fn list_users(query: &mut Request) -> JsonResult<UserListResponse> {
    let conn = db::pool();
    let query: UserListQuery = query.extract().await?;
    let username_filter = query.username.clone().unwrap_or_default();
    let like_pattern = format!("%{}%", username_filter);
    let offset = (query.current_page - 1) * query.page_size;
    
    let total = sqlx::query_scalar!(
        r#"
        SELECT COUNT(*) as "count!: i64" FROM users
        WHERE username LIKE $1
        "#,
        like_pattern
    )
    .fetch_one(conn)
    .await?;
    
    let users = sqlx::query_as!(
        SafeUser,
        r#"
        SELECT id, username FROM users
        WHERE username LIKE $1
        LIMIT $2 OFFSET $3
        "#,
        like_pattern,
        query.page_size,
        offset
    )
    .fetch_all(conn)
    .await?;
    
    json_ok(UserListResponse {
        data: users,
        total,
        current_page: query.current_page,
        page_size: query.page_size,
    })
}