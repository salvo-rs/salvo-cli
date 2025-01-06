use crate::config::CFG;
use salvo::cors::{AllowHeaders, AllowMethods, Cors, CorsHandler};

pub fn cors_hoop(config: &Config) -> CorsHandler {
    Cors::new()
        .allow_origin(&config.server.cors_allow_origin)
        .allow_methods(AllowMethods::any())
        .allow_headers(AllowHeaders::any())
        .into_handler()
}
