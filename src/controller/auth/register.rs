use serde::Deserialize;
use validator::Validate;

use crate::common::{error::ServerError, response::{ApiResponse, ApiResponseCode}, validate::ValidatedJson};



#[derive(Debug, Deserialize, Validate)]
pub struct RegisterByUsernameRequest {
  #[validate(length(min = 6, max = 50, message= "Username must be between 6 and 50 characters"))]
  pub username: String,
  #[validate(length(min = 6, max = 50, message= "Password must be between 6 and 50 characters"))]
  pub password: String,
}

pub async fn register_by_username(
  ValidatedJson(req): ValidatedJson<RegisterByUsernameRequest>
) -> Result<ApiResponse<bool>, ServerError> {
  Ok(ApiResponse{
    code: ApiResponseCode::Ok as i32,
    msg: None,
    data: Some(true)
  })
}