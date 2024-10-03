use axum::{extract::rejection::JsonRejection, response::{IntoResponse, Response}};
use axum_extra::extract::WithRejection;
use thiserror::Error;

use crate::response::{ApiError, ApiResponseCode};

#[derive(Debug, Error)]
pub enum MyRejection {
  #[error(transparent)]
  JSONExtractor(#[from] JsonRejection),
}

impl IntoResponse for MyRejection {
  fn into_response(self) -> Response {
    let err = match self {
      MyRejection::JSONExtractor(x) => match x {
        JsonRejection::JsonDataError(e) => {
          ApiError {
            code: ApiResponseCode::RequestError as i32,
            message: e.body_text(),
          }
        }
        JsonRejection::JsonSyntaxError(e) => {
          ApiError {
            code: ApiResponseCode::RequestError as i32,
            message: e.body_text(),
          }
        }
        JsonRejection::MissingJsonContentType(e) => {
          ApiError {
            code: ApiResponseCode::RequestError as i32,
            message: e.body_text(),
          }
        }
        _ => ApiError {
          code: ApiResponseCode::RequestError as i32,
          message: "Request error".to_string(),
        },
      },
    };
    err.into_response()
  }
}

pub type IRejection<T> = WithRejection<T, MyRejection>;