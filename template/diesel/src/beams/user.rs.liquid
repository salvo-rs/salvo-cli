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

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::users)]
pub struct User {
    pub id: String,
    pub username: String,
    pub password: String,
}

pub async fn add_user(req: UserAddRequest) -> AppResult<UserResponse> {
    use crate::schema::users;
    let mut db = establish_connection();
    let model = UserModel {
        id: Uuid::new_v4().to_string(),
        username: req.username.clone(),
        password: rand_utils::hash_password(req.password).await?,
    };

    diesel::insert_into(users::table)
        .values(&model)
        .execute(&mut db)
        .expect("Error saving new user");

    let user: UserModel = users::table
        .filter(users::id.eq(&model.id))
        .first(&mut db)
        .expect("Error loading user");

    Ok(UserResponse {
        id: user.id,
        username: user.username,
    })
}

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

pub async fn update_user(req: UserUpdateRequest) -> AppResult<UserResponse> {
    let mut db = establish_connection();
    let user = diesel::update(diesel_users.find(req.id))
        .set((
            username.eq(req.username),
            password.eq(rand_utils::hash_password(req.password).await?),
        ))
        .returning(UserModel::as_returning())
        .get_result(&mut db)?;
    Ok(UserResponse {
        id: user.id,
        username: user.username,
    })
}

pub async fn delete_user(req: String) -> AppResult<()> {
    use crate::schema::users::id;
    let mut db = establish_connection();
    diesel::delete(diesel_users.filter(id.eq(req)))
        .execute(&mut db)
        .expect("Error deleting posts");
    Ok(())
}

pub async fn users() -> AppResult<Vec<UserResponse>> {
    let mut db = establish_connection();
    let results = diesel_users.select(UserModel::as_select()).load(&mut db)?;

    let res = results
        .into_iter()
        .map(|user| UserResponse {
            id: user.id,
            username: user.username,
        })
        .collect::<Vec<_>>();

    Ok(res)
}