use std::str::FromStr;

use futures_util::StreamExt;
use mongodb::bson::{doc, oid::ObjectId};
use rinja::Template;
use salvo::oapi::extract::*;
use salvo::prelude::*;
use serde::{Deserialize,Serialize};
use validator::Validate;
use crate::hoops::jwt;

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
    if let Some(cookie) = res.cookies().get("jwt_token") {
        let token = cookie.value().to_string();
        if !jwt::decode_token(&token) {
            res.render(Redirect::other("/login"));
        }
    }
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
    let coll_users = db::users();
    let user = doc! {
        "username": username,
        "password": utils::hash_password(&password)?
    };
    coll_users.insert_one(user.clone()).await?;
    let Some(user) = coll_users.find_one(user).await? else {
        return Err(StatusError::bad_request()
            .brief("User does not exists.")
            .into());
    };
    json_ok(SafeUser {
        id: user.get_object_id("_id")?.to_string(),
        username: user.get_str("username")?.to_string(),
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
    let UpdateInData { username, password } = idata.into_inner();
    let password = utils::hash_password(&password)?;
    let coll_users = db::users();
    coll_users
        .update_one(
            doc! { "_id": ObjectId::from_str(&user_id)? },
            doc! { "$set": { "username": &username, "password": password } },
        )
        .await?;
    json_ok(SafeUser {
        id: user_id,
        username,
    })
}

#[endpoint(tags("users"))]
pub async fn delete_user(user_id: PathParam<String>) -> EmptyResult {
    let user_id = user_id.into_inner();
    let coll_users = db::users();
    coll_users
        .delete_one(doc! { "_id": ObjectId::from_str(&user_id)? })
        .await?;
    empty_ok()
}

#[derive(Debug, Deserialize, Validate, Extractible, ToSchema)]
#[salvo(extract(default_source(from = "query")))]
pub struct UserListQuery {
    pub username: Option<String>,
    #[serde(default = "default_page")]
    pub current_page: u64,
    #[serde(default = "default_page_size")]
    pub page_size: u64,
}

fn default_page() -> u64 { 1 }
fn default_page_size() -> u64 { 10 }

#[derive(Debug, Serialize, ToSchema)]
pub struct UserListResponse {
    pub data: Vec<SafeUser>,
    pub total: u64,
    pub current_page: u64,
    pub page_size: u64,
}

#[endpoint(tags("users"))]
pub async fn list_users(query: &mut Request) -> JsonResult<UserListResponse> {
    let query: UserListQuery = query.extract().await?;
    let coll_users = db::users();

    let mut filter = doc! {};
    if let Some(username) = &query.username {
        filter.insert("username", doc! { "$regex": username, "$options": "i" });
    }

    let total = coll_users.count_documents(filter.clone()).await?;
    let mut cursor = coll_users
        .find(filter)
        .await?
        .skip(((query.current_page - 1) * query.page_size).try_into().unwrap())
        .take(query.page_size as usize);

    let mut users = Vec::new();
    while let Some(result) = cursor.next().await {
        let document = result?;
        let id = document.get_object_id("_id")?.to_string();
        let username = document.get_str("username")?.to_owned();
        users.push(SafeUser { id, username });
    }

    json_ok(UserListResponse {
        data: users,
        total,
        current_page: query.current_page,
        page_size: query.page_size,
    })
}
