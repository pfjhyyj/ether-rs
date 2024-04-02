use serde::Deserialize;
use validator::Validate;


#[derive(Debug, Deserialize, Validate)]
pub struct LoginByUserNameRequest {
  #[validate(length(min = 6, max = 50, message= "Username must be between 6 and 50 characters"))]
  pub username: String,
  #[validate(length(min = 6, max = 50, message= "Password must be between 6 and 50 characters"))]
  pub password: String,
}

pub async fn login_by_username() {
  // Code goes here
}