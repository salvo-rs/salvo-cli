use salvo::{
    async_trait, http::ParseError, prelude::EndpointOutRegister, writing::Json, Depot, Request,
    Response, Writer,
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("error:`{0}`")]
    AnyHow(#[from] anyhow::Error),
    #[error("http::ParseError:`{0}`")]
    ParseError(#[from] ParseError),
}

pub type AppResult<T> = Result<T, AppError>;

#[async_trait]
impl Writer for AppError {
    async fn write(mut self, _req: &mut Request, _depot: &mut Depot, res: &mut Response) {
        res.render(Json(self.to_string()));
    }
}

impl EndpointOutRegister for AppError {
    fn register(_components: &mut salvo::oapi::Components, operation: &mut salvo::oapi::Operation) {
        operation
            .responses
            .insert("500".to_string(), salvo::oapi::Response::new("error"));
    }
}
