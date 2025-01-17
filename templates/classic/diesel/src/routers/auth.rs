use crate::{hoops::jwt::decode_token, labors::user};
use crate::{json_empty, json_ok, AppResult, JsonResult};
use askama::Template;
use salvo::prelude::*;

#[endpoint]
pub async fn show_login(res: &mut Response) -> AppResult<Text> {
    #[derive(Template)]
    #[template(path = "login.html")]
    struct LoginTemplate {}
    if let Some(cookie) = res.cookies().get("jwt_token") {
        let token = cookie.value().to_string();
        if decode_token(&token) {
            res.render(Redirect::other("/users"));
            return Ok(());
        }
    }
    let hello_tmpl = LoginTemplate {};
    Ok(Text::Html(hello_tmpl.render().unwrap()))
}
#[derive(Debug, Serialize, ToSchema, Default)]
pub struct LoginInData {
    pub id: String,
    pub username: String,
    pub token: String,
    pub exp: i64,
}
#[endpoint(tags("auth"))]
pub async fn post_login(req: JsonBody<LoginInData>, res: &mut Response) -> JsonResult<()> {
    let result = diesel_users
        .filter(username.eq(&req.username))
        .select((id, username, password))
        .first::<(String, String, String)>(&mut connection)
        .optional()?;

    match result {
        None => Err(anyhow::anyhow!("User does not exist.").into()),
        Some((uid, uname, hashed_pwd)) => {
            if rand_utils::verify_password(req.password, hashed_pwd)
                .await
                .is_err()
            {
                return Err(anyhow::anyhow!("Incorrect password.").into());
            }

            let (token, exp) = get_token(uname.clone(), uid.clone())?;
            let res = UserLoginResponse {
                id: uid,
                username: uname,
                token,
                exp,
            };
            Ok(res)
        }
    }
    let jwt_token = data.token.clone();
    let cookie = Cookie::build(("jwt_token", jwt_token))
        .path("/")
        .http_only(true)
        .build();
    res.add_cookie(cookie);
    json_empty()
}
