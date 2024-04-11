use serde::Serialize;

use crate::common::{error::ServerError, response::ApiResponse};


#[derive(Debug, Serialize)]
pub struct GetCurrentUserResponse {
  pub user_id: i64,
  pub username: String,
  pub avatar: String,
}

pub async fn get_current_user() -> Result<ApiResponse<GetCurrentUserResponse>, ServerError> {
  let resp = GetCurrentUserResponse{
    user_id: 1,
    username: "test".to_string(),
    avatar: "avatar".to_string()
  };

  Ok(ApiResponse::ok(resp))
}