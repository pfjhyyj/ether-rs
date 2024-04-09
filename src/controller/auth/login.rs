use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::common::{error::ServerError, response::{ApiResponse, ApiResponseCode}, validate::ValidatedJson};


#[derive(Debug, Deserialize, Validate)]
pub struct LoginByUserNameRequest {
  #[validate(length(min = 6, max = 50, message= "Username must be between 6 and 50 characters"))]
  pub username: String,
  #[validate(length(min = 6, max = 50, message= "Password must be between 6 and 50 characters"))]
  pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginByUserNameResponse {
  pub access_token: String,
  pub expire_time: String
}

pub async fn login_by_username(
  ValidatedJson(req): ValidatedJson<LoginByUserNameRequest>
) -> Result<ApiResponse<LoginByUserNameResponse>, ServerError> {
  if req.password.contains("123") {
    println!("ok")
  };
  let resp = LoginByUserNameResponse{
    access_token: "321".to_string(),
    expire_time: "123".to_string(),
  };

  Ok(ApiResponse{
    code: ApiResponseCode::Ok as i32,
    msg: None,
    data: Some(resp)
  })
}