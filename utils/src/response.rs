use axum::{
    response::{IntoResponse, Response},
    Json,
};
use http::StatusCode;
use serde::Serialize;

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
    pub message: Option<String>,
    pub data: Option<T>,
}

impl<T> ApiResponse<T> {
    pub fn ok(data: T) -> Self {
        Self {
            code: ApiResponseCode::Ok as i32,
            message: None,
            data: Some(data),
        }
    }
}

impl ApiResponse<()> {
    pub fn error(code: i32, message: String) -> Self {
        Self {
            code,
            message: Some(message),
            data: None,
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

pub struct ApiOk<T> {
    pub data: T,
}

impl<T> ApiOk<T> {
    pub fn new(data: T) -> Self {
        Self { data }
    }
}

impl<T> IntoResponse for ApiOk<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        ApiResponse::ok(self.data).into_response()
    }
}

pub struct ApiError {
    pub code: i32,
    pub message: String,
}

impl ApiError {
    pub fn new(code: i32, message: String) -> Self {
        Self { code, message }
    }

    pub fn err_param(message: String) -> Self {
        Self {
            code: ApiResponseCode::RequestError as i32,
            message,
        }
    }

    pub fn err_db() -> Self {
        Self {
            code: ApiResponseCode::DbError as i32,
            message: "An error occurred. Please try again later.".to_string(),
        }
    }

    pub fn err_unknown() -> Self {
        Self {
            code: ApiResponseCode::UnknownError as i32,
            message: "An error occurred. Please try again later.".to_string(),
        }
    }

    pub fn err_unauthenticated() -> Self {
        Self {
            code: ApiResponseCode::AuthError as i32,
            message: "Unauthenticated. Please login".to_string(),
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        ApiResponse::error(self.code, self.message).into_response()
    }
}

pub type Result<T> = std::result::Result<T, ApiError>;

#[derive(Debug, Serialize)]
pub struct PageResponse<T> {
    pub total: u64,
    pub data: Vec<T>,
}
