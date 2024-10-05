use axum::{
    async_trait,
    extract::{rejection::JsonRejection, FromRequest, Request},
    Json,
};
use serde::de::DeserializeOwned;
use validator::Validate;

use crate::response::{ApiError, ApiResponseCode};

#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedJson<T>(pub T);

#[async_trait]
impl<T, S> FromRequest<S> for ValidatedJson<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
    Json<T>: FromRequest<S, Rejection = JsonRejection>,
{
    type Rejection = ApiError;

    async fn from_request(req: Request, state: &S) -> Result<Self, ApiError> {
        let Json(value) = Json::<T>::from_request(req, state).await.map_err(|err| {
            ApiError::new(
                ApiResponseCode::RequestError as i32,
                format!("JSON parsing error: {}", err),
            )
        })?;
        value.validate().map_err(|e| {
            ApiError::new(
                ApiResponseCode::RequestError as i32,
                format!("Validation error: {}", e),
            )
        })?;
        Ok(ValidatedJson(value))
    }
}
