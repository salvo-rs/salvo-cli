use askama::Template;
use salvo::{handler, prelude::StatusCode, writing::Text, Depot, FlowCtrl, Request, Response};

pub mod jwt;
pub use jwt::jwt_hoop;
mod cors;
pub use cors::cors_hoop;

#[derive(Template)]
#[template(path = "error_404.html")]
struct Error404;

#[handler]
pub async fn error_404(&self, res: &mut Response, ctrl: &mut FlowCtrl) {
    if let Some(StatusCode::NOT_FOUND) = res.status_code {
        let handle404 = Error404;
        res.render(Text::Html(handle404.render().unwrap()));
        ctrl.skip_rest();
    }
}
