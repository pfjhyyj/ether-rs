use axum::Json;
use axum_extra::extract::WithRejection;
use serde::{Deserialize, Serialize};
use utils::{rejection::IRejection, response::{ApiError, ApiOk, Result}};
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct LoginByUserNameRequest {
    #[validate(length(
        min = 6,
        max = 50,
        message = "Username must be between 6 and 50 characters"
    ))]
    pub username: String,
    #[validate(length(
        min = 6,
        max = 50,
        message = "Password must be between 6 and 50 characters"
    ))]
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginByUserNameResponse {
    pub access_token: String,
    pub expire_time: String,
}

pub async fn login_by_username(
  WithRejection(Json(req), _): IRejection<Json<LoginByUserNameRequest>>,
) -> Result<ApiOk<LoginByUserNameResponse>> {
    if let Err(e) = req.validate() {
        return Err(ApiError::err_param(e.to_string()))
    }

    let resp = LoginByUserNameResponse {
        access_token: "123".to_string(),
        expire_time: "123".to_string(),
    };

    Ok(ApiOk::new(resp))
}