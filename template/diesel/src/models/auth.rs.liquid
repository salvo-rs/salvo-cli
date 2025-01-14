use diesel::prelude::*;

use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use uuid::Uuid;

use crate::schema::users::dsl::users as diesel_users;
use crate::{
    app_writer::AppResult,
    db::establish_connection,
    dtos::user::{
        UserAddRequest, UserLoginRequest, UserLoginResponse, UserResponse, UserUpdateRequest,
    },
    middleware::jwt::get_token,
    models::user::UserModel,
    utils::rand_utils,
};
use diesel::OptionalExtension;

pub async fn login(req: UserLoginRequest) -> AppResult<UserLoginResponse> {
    use crate::schema::users::dsl::*;
    let mut connection = establish_connection();

    let result = diesel_users
        .filter(username.eq(&req.username))
        .select((id, username, password))
        .first::<(String, String, String)>(&mut connection)
        .optional()?;

    match result {
        None => Err(anyhow::anyhow!("{{user_does_not_exist}}").into()),
        Some((uid, uname, hashed_pwd)) => {
            if rand_utils::verify_password(req.password, hashed_pwd)
                .await
                .is_err()
            {
                return Err(anyhow::anyhow!("{{incorrect_password}}").into());
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
}