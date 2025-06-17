use cookie::Cookie;
use mongodb::bson::doc;
use rinja::Template;
use salvo::oapi::extract::*;
use salvo::prelude::*;
use serde::{Deserialize, Serialize};

use crate::hoops::jwt;
use crate::{db, json_ok, utils, AppResult, JsonResult};

#[handler]
pub async fn login_page(res: &mut Response) -> AppResult<()> {
    #[derive(Template)]
    #[template(path = "login.html")]
    struct LoginTemplate {}
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
    let idata = idata.into_inner();
    let coll_users = db::users();

    let Some(user) = coll_users
        .find_one(doc! { "username": &idata.username })
        .await?
    else {
        return Err(StatusError::unauthorized()
            .brief("User does not exist.")
            .into());
    };

    if utils::verify_password(&idata.password, &user.get_str("password")?.to_string()).is_err() {
        return Err(StatusError::unauthorized()
            .brief("Account not exist or password is incorrect.")
            .into());
    }

    let (token, exp) = jwt::get_token(user.get_object_id("_id")?.to_string())?;
    let odata = LoginOutData {
        id: user.get_object_id("_id")?.to_string(),
        username: user.get_str("username")?.to_owned(),
        token,
        exp,
    };
    let cookie = Cookie::build(("jwt_token", odata.token.clone()))
        .path("/")
        .http_only(true)
        .build();
    res.add_cookie(cookie);
    json_ok(odata)
}
