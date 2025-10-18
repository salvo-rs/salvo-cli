use anyhow::Result;
use cookie::Cookie;
use rbs::value;
use rinja::Template;
use salvo::oapi::extract::*;
use salvo::prelude::*;
use serde::{Deserialize, Serialize};

use crate::hoops::jwt;
use crate::models::User;
use crate::{db, json_ok, utils, JsonResult};

#[derive(Template)]
#[template(path = "login.html")]
struct LoginTemplate {}

#[handler]
pub async fn login_page(res: &mut Response) -> Result<()> {
    if let Some(cookie) = res.cookies().get("jwt_token") {
        let token = cookie.value().to_string();
        if jwt::decode_token(&token) {
            res.render(Redirect::other("/users"));
            return Ok(());
        }
    }
    let hello_tmpl = LoginTemplate {};
    res.render(Text::Html(hello_tmpl.render().unwrap()));
    Ok(())
}

#[derive(Deserialize, ToSchema, Default, Debug)]
pub struct LoginInData {
    pub username: String,
    pub password: String,
}
#[derive(Serialize, ToSchema, Default, Debug)]
pub struct LoginOutData {
    pub id: String,
    pub username: String,
    pub token: String,
    pub exp: i64,
}
#[endpoint(tags("auth"))]
pub async fn post_login(
    idata: JsonBody<LoginInData>,
    res: &mut Response,
) -> JsonResult<LoginOutData> {
    let login_data = idata.into_inner();
    let rb = db::engine();

    // Find user by username
    let users = User::select_by_map(rb, value!("username": &login_data.username))
        .await
        .map_err(anyhow::Error::from)?;

    let user = users
        .first()
        .ok_or_else(|| StatusError::unauthorized().brief("User does not exist."))?;

    // Verify password
    if utils::verify_password(&login_data.password, &user.password).is_err() {
        return Err(StatusError::unauthorized()
            .brief("Account not exist or password is incorrect.")
            .into());
    }

    // Generate JWT token - using user ID as the token identifier
    let (token, exp) = jwt::get_token(&user.id)?;

    let odata = LoginOutData {
        id: user.id.to_string(),
        username: user.username.to_string(),
        token: token.clone(),
        exp,
    };

    let cookie = Cookie::build(("jwt_token", token))
        .path("/")
        .http_only(true)
        .build();
    res.add_cookie(cookie);

    json_ok(odata)
}
