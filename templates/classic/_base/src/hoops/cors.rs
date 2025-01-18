use salvo::cors::{AllowHeaders, AllowMethods, AllowOrigin, Cors, CorsHandler};

pub fn cors_hoop() -> CorsHandler {
    Cors::new()
        .allow_origin(AllowOrigin::any())
        .allow_methods(AllowMethods::any())
        .allow_headers(AllowHeaders::any())
        .into_handler()
}
