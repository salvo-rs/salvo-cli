use rust_embed::RustEmbed;
use salvo::prelude::*;
use salvo::serve_static::static_embed;

mod auth;
mod demo;
mod user;

#[derive(RustEmbed)]
#[folder = "assets"]
struct Assets;

pub fn root() -> Router {
    let router = Router::new()
        .hoop(Logger::new())
        .get(demo::hello)
        .push(Router::with_path("/api/login").post(auth::post_login))
        .push(
            Router::with_path("/api/users")
                // .hoop(auth())
                .get(user::list_users)
                .post(user::create_user)
                .push(
                    Router::with_path("{id}")
                        .put(user::update_user)
                        .delete(user::delete_user),
                ),
        )
        .push(Router::with_path("assets/{**rest}").get(static_embed::<Assets>()));
    let doc = OpenApi::new("salvo web api", "0.0.1").merge_router(&router);
    router
        .unshift(doc.into_router("/api-doc/openapi.json"))
        .unshift(Scalar::new("/api-doc/openapi.json").into_router("scalar"))
}
