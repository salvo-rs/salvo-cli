use rinja::Template;
use salvo::http::ResBody;
use salvo::prelude::*;

pub mod custom_middleware_example;
pub mod jwt;
pub use jwt::auth_hoop;
mod cors;
pub use cors::cors_hoop;

#[derive(Template)]
#[template(path = "error_404.html")]
struct Error404 {
    brief: String,
}

#[handler]
pub async fn error_404(&self, res: &mut Response, ctrl: &mut FlowCtrl) {
    if let Some(StatusCode::NOT_FOUND) = res.status_code {
        let handle404 = Error404 {
            brief: if let ResBody::Error(e) = &res.body {
                e.brief.clone()
            } else {
                "Page not found".to_owned()
            },
        };
        res.render(Text::Html(handle404.render().unwrap()));
        ctrl.skip_rest();
    }
}
