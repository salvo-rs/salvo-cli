use salvo::prelude::*;
use salvo::serve_static::static_embed;

mod demo;
mod user;

#[derive(RustEmbed)]
#[folder = "assets"]
struct Assets;

pub fn root() -> Router {
    let router = Router::new()
        .hoop(Logger::new())
        .hoop(CatchPanic::new())
        .get(hello)
        .push(Router::with_path("/api/login").post(post_login))
        .push(
            Router::with_path("/api/users")
                .hoop(auth())
                .get(get_users)
                .post(post_add_user)
                .push(
                    Router::with_path("{id}")
                        .put(put_update_user)
                        .delete(delete_user),
                ),
        )
        .push(Router::with_path("assets/{**rest}").get(static_embed::<Assets>()));
    let doc = OpenApi::new("salvo web api", "0.0.1").merge_router(&router);
    router
        .unshift(doc.into_router("/api-doc/openapi.json"))
        .unshift(Scalar::new("/api-doc/openapi.json").into_router("scalar"))
}
