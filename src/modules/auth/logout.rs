use crate::common::{error::ServerError, response::ApiResponse};



pub async fn logout() -> Result<ApiResponse<bool>, ServerError> {
  Ok(ApiResponse::ok(true))
}