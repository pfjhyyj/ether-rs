use sea_orm::{Set, ActiveModelTrait};
use serde::Deserialize;
use validator::Validate;
use entity::user;
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
) -> Result<ApiResponse<i64>, ServerError> {

  let new_user = create_user_by_register_request(req).await?;

  Ok(ApiResponse{
    code: ApiResponseCode::Ok as i32,
    msg: None,
    data: Some(new_user.user_id)
  })
}

async fn create_user_by_register_request(req: RegisterByUsernameRequest) -> Result<user::Model, ServerError> {
  let db = client::database::get_db_connection().await;

  let hashed_password = bcrypt::hash(req.password, bcrypt::DEFAULT_COST).expect("Failed to hash password");

  let new_user = user::ActiveModel {
    username: Set(req.username),
    password: Set(hashed_password),
    ..Default::default()
  };

  let new_user: user::Model = new_user.insert(db).await?;

  Ok(new_user)
}