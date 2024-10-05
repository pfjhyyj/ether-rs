use axum::Extension;
use entity::user;
use sea_orm::{EntityTrait, QueryFilter, ColumnTrait};
use serde::Serialize;
use utils::{middleware::jwt::Claims, response::{ApiError, ApiOk, Result}};


#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCurrentUserResponse {
  pub user_id: i64,
  pub username: String,
  pub nickname: Option<String>,
  pub avatar: Option<String>,
}

pub async fn get_current_user(
  Extension(token_data): Extension<Claims>,
) -> Result<ApiOk<GetCurrentUserResponse>> {
  let user = get_user_by_user_id(token_data.sub).await?;

  let resp = GetCurrentUserResponse {
    user_id: user.user_id,
    username: user.username,
    nickname: user.nickname,
    avatar: user.avatar
  };

  Ok(ApiOk::new(resp))
}

async fn get_user_by_user_id(user_id: i64) -> Result<user::Model> {
  let db = utils::db::conn();
  let user = user::Entity::find()
    .filter(user::Column::UserId.eq(user_id))
    .one(db)
    .await
    .map_err(|e| {
      tracing::error!(error = ?e, "Failed to query user by user_id");
      ApiError::err_db("Failed to query user by user_id".to_string())
    })?;
  
  user.ok_or(ApiError::err_param("User not found".to_string()))
}