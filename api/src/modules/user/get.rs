use axum::extract::Path;
use entity::user;
use sea_orm::EntityTrait;
use serde::Serialize;
use utils::response::{ApiError, ApiOk, Result};

#[derive(Debug, Serialize)]
pub struct GetUserDetailResponse {
    pub user_id: i64,
    pub username: String,
    pub email: Option<String>,
    pub nickname: Option<String>,
    pub avatar: Option<String>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

pub async fn get_user(
    Path(user_id): Path<i64>,
) -> Result<ApiOk<GetUserDetailResponse>> {
    let user = get_user_by_id(user_id).await?;
    let user = GetUserDetailResponse {
        user_id: user.user_id,
        username: user.username,
        email: user.email,
        nickname: user.nickname,
        avatar: user.avatar,
        created_at: user.created_at.naive_local(),
        updated_at: user.updated_at.naive_local(),
    };

    Ok(ApiOk::new(user))
}

async fn get_user_by_id(id: i64) -> Result<user::Model> {
    let db = utils::db::conn();
    let user = user::Entity::find_by_id(id)
        .one(db)
        .await
        .map_err(|e| {
            tracing::error!(error = ?e, "Failed to find user");
            ApiError::err_db()
        })?;

    if let Some(user) = user {
        Ok(user)
    } else {
        Err(ApiError::err_param("User not found".to_string()))
    }
}