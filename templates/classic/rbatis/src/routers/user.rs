use rbatis::impl_select_page;
use rbatis::plugin::page::PageRequest;
use rinja::Template;
use salvo::oapi::extract::*;
use salvo::prelude::*;
use serde::{Deserialize, Serialize};
use ulid::Ulid;
use validator::Validate;

use crate::models::{SafeUser, User};
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
    let rb = db::engine();

    let id = Ulid::new().to_string();
    let user = User {
        id: id.clone(),
        username: username.clone(),
        password: utils::hash_password(&password)?,
    };

    User::insert(rb, &user).await.map_err(anyhow::Error::from)?;

    json_ok(SafeUser {
        id: id,
        username: username,
    })
}

#[derive(Deserialize, Debug, Validate, ToSchema)]
struct UpdateInData {
    #[validate(length(min = 5, message = "username length must be greater than 5"))]
    username: String,
    #[validate(length(min = 6, message = "password length must be greater than 5"))]
    password: String,
}
#[endpoint(tags("users"), parameters(("user_id", description = "user id")))]
pub async fn update_user(
    user_id: PathParam<String>,
    idata: JsonBody<UpdateInData>,
) -> JsonResult<SafeUser> {
    let user_id = user_id.into_inner();
    let idata = idata.into_inner();
    let rb = db::engine();

    let user = User {
        id: user_id.clone(),
        username: idata.username.clone(),
        password: utils::hash_password(&idata.password)?,
    };

    User::update_by_column(rb, &user, "id")
        .await
        .map_err(anyhow::Error::from)?;

    json_ok(SafeUser {
        id: user_id,
        username: idata.username,
    })
}

#[endpoint(tags("users"))]
pub async fn delete_user(user_id: PathParam<String>) -> EmptyResult {
    let rb = db::engine();
    User::delete_by_column(rb, "id", &user_id.into_inner())
        .await
        .map_err(anyhow::Error::from)?;
    empty_ok()
}

impl_select_page!(User{select_page() =>"
     if !sql.contains('count(1)'):
       `order by id desc`"});

impl_select_page!(User{select_page_by_username(username:&str) =>"
     if username != null && username != '':
       `where username like #{username}`"});

#[derive(Debug, Deserialize, Validate, Extractible, ToSchema)]
#[salvo(extract(default_source(from = "query")))]
pub struct UserListQuery {
    pub username: Option<String>,
    #[serde(default = "default_page")]
    pub current_page: u64,
    #[serde(default = "default_page_size")]
    pub page_size: u64,
}

fn default_page() -> u64 {
    1
}
fn default_page_size() -> u64 {
    10
}

#[derive(Debug, Serialize, ToSchema)]
pub struct UserListResponse {
    pub data: Vec<SafeUser>,
    pub total: i64,
    pub current_page: u64,
    pub page_size: u64,
}

#[endpoint(tags("users"), status_codes(200, 400))]
pub async fn list_users(query: &mut Request) -> JsonResult<UserListResponse> {
    let rb = db::engine();
    let query: UserListQuery = query.extract().await?;

    let page_req = PageRequest::new(query.current_page, query.page_size);

    let page = if let Some(username) = query.username {
        let pattern = format!("%{}%", username);
        User::select_page_by_username(rb, &page_req, &pattern)
            .await
            .map_err(anyhow::Error::from)?
    } else {
        User::select_page(rb, &page_req)
            .await
            .map_err(anyhow::Error::from)?
    };

    let safe_users: Vec<SafeUser> = page
        .records
        .into_iter()
        .map(|user| SafeUser {
            id: user.id,
            username: user.username,
        })
        .collect();

    json_ok(UserListResponse {
        data: safe_users,
        total: page.total as i64,
        current_page: query.current_page,
        page_size: query.page_size,
    })
}
