use axum::Extension;
use entity::user;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, ActiveModelTrait, Set};
use serde::{Deserialize, Serialize};
use utils::{
    middleware::jwt::Claims, rejection::ValidatedJson, response::{ApiError, ApiOk, Result}
};
use validator::Validate;

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
        avatar: user.avatar,
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
            ApiError::err_db()
        })?;

    user.ok_or(ApiError::err_param("User not found".to_string()))
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateCurrentUserRequest {
    pub nickname: Option<String>,
    pub avatar: Option<String>,
}

pub async fn update_current_user(
    Extension(token_data): Extension<Claims>,
    ValidatedJson(req): ValidatedJson<UpdateCurrentUserRequest>,
) -> Result<ApiOk<GetCurrentUserResponse>> {
    let user = update_user_by_user_id(token_data.sub, req).await?;

    let resp = GetCurrentUserResponse {
        user_id: user.user_id,
        username: user.username,
        nickname: user.nickname,
        avatar: user.avatar,
    };

    Ok(ApiOk::new(resp))
}

async fn update_user_by_user_id(user_id: i64, req: UpdateCurrentUserRequest) -> Result<user::Model> {
    let db = utils::db::conn();
    let user = user::Entity::find()
        .filter(user::Column::UserId.eq(user_id))
        .one(db)
        .await
        .map_err(|e| {
            tracing::error!(error = ?e, "Failed to query user by user_id");
            ApiError::err_db()
        })?;

    let user = user.ok_or(ApiError::err_param("User not found".to_string()))?;

    let mut user: user::ActiveModel = user.into();
    if req.nickname.is_some() {
        user.nickname = Set(req.nickname.to_owned());
    }
    if req.avatar.is_some() {
        user.avatar = Set(req.avatar.to_owned());
    }

    let user = user.update(db).await.map_err(|e| {
        tracing::error!(error = ?e, "Failed to update user by user_id");
        ApiError::err_db()
    })?;
    Ok(user)
}
