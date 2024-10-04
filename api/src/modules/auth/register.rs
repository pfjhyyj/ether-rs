use entity::user;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, Set, ActiveModelTrait};
use serde::Deserialize;
use utils::{rejection::ValidatedJson, response::{ApiError, ApiOk, Result}};
use validator::Validate;


#[derive(Debug, Deserialize, Validate)]
pub struct RegisterByUsernameRequest {
  #[validate(length(min = 6, max = 50, message= "Username must be between 6 and 50 characters"))]
  pub username: String,
  #[validate(length(min = 6, max = 50, message= "Password must be between 6 and 50 characters"))]
  pub password: String,
}

pub async fn register_by_username(
  ValidatedJson(req): ValidatedJson<RegisterByUsernameRequest>
) -> Result<ApiOk<i64>> {
  let new_user = create_user_by_register_request(req).await?;

  Ok(ApiOk::new(new_user.user_id))
}

async fn create_user_by_register_request(req: RegisterByUsernameRequest) -> Result<user::Model> {
  let db = utils::db::conn();
  let user = user::Entity::find()
    .filter(user::Column::Username.eq(&req.username))
    .one(db)
    .await
    .map_err(|e| {
      tracing::error!(error = ?e, "Failed to query user by username");
      ApiError::err_db("Failed to query user by username".to_string())
    })?;

  if user.is_some() {
    return Err(ApiError::err_param("Username already exists".to_string()));
  }

  let password = utils::hash::bcrypt(&req.password);

  let new_user = user::ActiveModel {
    username: Set(req.username),
    password: Set(password),
    ..Default::default()
  };

  let new_user = new_user
    .insert(db)
    .await
    .map_err(|e| {
      tracing::error!(error = ?e, "Failed to insert new user");
      ApiError::err_db("Failed to create new user".to_string())
    })?;

  Ok(new_user)
}