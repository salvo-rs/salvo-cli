use salvo::{
    async_trait,
    prelude::EndpointOutRegister,
    writing::Json,
    Depot, Request, Response, Writer, hyper::StatusCode,
};
use serde::Serialize;

use crate::app_error::AppError;

pub struct AppResponse<T>(pub AppResult<T>);

#[async_trait]
impl<T: Serialize + Default + Send> Writer for AppResponse<T> {
    async fn write(self, req: &mut Request, depot: &mut Depot, res: &mut Response) {
        match self.0 {
            Ok(data) => Res::with_data(data).into_response(res),
            Err(e) => e.write(req, depot, res).await,
        }
    }
}

impl<T: Serialize + Default + Send> EndpointOutRegister for AppResponse<T> {
    fn register(_components: &mut salvo::oapi::Components, operation: &mut salvo::oapi::Operation) {
        operation
            .responses
            .insert("0".to_string(), salvo::oapi::Response::new("success"));
        operation
            .responses
            .insert("500".to_string(), salvo::oapi::Response::new("error"));
    }
}

impl<T> From<AppResult<T>> for AppResponse<T> {
    fn from(result: AppResult<T>) -> Self {
        AppResponse(result)
    }
}

impl<T> From<AppError> for AppResponse<T> {
    fn from(result: AppError) -> Self {
        AppResponse(Err(result))
    }
}

#[derive(Debug, Serialize, Default)]
pub struct Res<T> {
    pub code: i32,
    pub data: T,
    pub msg: String,
}

#[derive(Debug, Serialize, Default)]
pub struct ErrRes {
    pub code: i32,
    pub msg: String,
}

impl<T: Serialize + Send + Default> Res<T> {
    pub fn with_data(data: T) -> Self {
        Self {
            code: 0,
            data,
            msg: "success".to_string(),
        }
    }
    #[allow(dead_code)]
    pub fn with_data_msg(data: T, msg: &str) -> Self {
        Self {
            code: 0,
            data,
            msg: msg.to_string(),
        }
    }
}

impl ErrRes {
    pub fn with_err(err: &str) -> Self {
        Self {
            code: 500,
            msg: err.to_string(),
        }
    }
}
impl<T: Serialize + Send + Default> Res<T> {
    pub fn into_response(self, res: &mut Response) {
        res.render(Json(self));
    }
}

impl ErrRes {
    pub fn into_response(self, res: &mut Response) {
        res.stuff(StatusCode::INTERNAL_SERVER_ERROR, Json(self));
    }
}

pub type AppResult<T> = Result<T, AppError>;

#[async_trait]
impl Writer for AppError {
    async fn write(mut self, _req: &mut Request, _depot: &mut Depot, res: &mut Response) {
        ErrRes::with_err(&self.to_string()).into_response(res)
    }
}

impl EndpointOutRegister for AppError {
    fn register(_components: &mut salvo::oapi::Components, operation: &mut salvo::oapi::Operation) {
        operation
            .responses
            .insert("500".to_string(), salvo::oapi::Response::new("error"));
    }
}
