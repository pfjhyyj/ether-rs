use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

use super::error::ServerError;

pub enum ApiResponseCode {
    Ok = 0,
    UnknownError = 10001,
    DbError = 10002,

    RequestError = 20001,
    AuthError = 20002,
    AuthorizedError = 20003,
}

#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub code: i32,
    pub msg: Option<String>,
    pub data: Option<T>,
}

impl<T> ApiResponse<T> {
    pub fn ok(data: T) -> ApiResponse<T> {
        ApiResponse {
            code: ApiResponseCode::Ok as i32,
            msg: None,
            data: Some(data),
        }
    }
}

impl<T> IntoResponse for ApiResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        match self {
            ServerError::ValidationError(_) => {
                let message = format!("{self}").replace('\n', ", ");
                let resp = ApiResponse::<String> {
                    code: ApiResponseCode::RequestError as i32,
                    msg: Some(message),
                    data: None,
                };
                (StatusCode::OK, Json(resp))
            }
            _ => {
                let resp = ApiResponse::<String> {
                    code: ApiResponseCode::RequestError as i32,
                    msg: Some(self.to_string()),
                    data: None,
                };
                (StatusCode::OK, Json(resp))
            }
        }
        .into_response()
    }
}
