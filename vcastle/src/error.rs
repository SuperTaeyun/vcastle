use axum::response::{IntoResponse, Response};
use axum::Json;
use thiserror::Error;

pub type ApiResult<T> = Result<T, ServerError>;

pub type JsonResult<T> = ApiResult<Json<T>>;

#[derive(Error, Debug)]
pub enum ServerError {}

pub struct ErrorResponse {}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> Response {
        todo!()
    }
}
