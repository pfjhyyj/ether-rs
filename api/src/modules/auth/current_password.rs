use axum::Extension;
use entity::user;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, ActiveModelTrait, Set};
use serde::Deserialize;
use utils::{
    middleware::jwt::Claims, rejection::ValidatedJson, response::{ApiError, ApiOk, Result}
};
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UpdatePasswordRequest {
    pub old_password: String,
    pub new_password: String,
    pub confirm_password: String,
}

pub async fn update_password(
    Extension(token_data): Extension<Claims>,
    ValidatedJson(req): ValidatedJson<UpdatePasswordRequest>,
) -> Result<ApiOk<()>> {
    let user = get_user_by_user_id(token_data.sub).await?;

    let is_valid = utils::hash::bcrypt_verify(&req.old_password, &user.password);
    if !is_valid {
        return Err(ApiError::err_param("Invalid old password".to_string()));
    }

    if req.new_password != req.confirm_password {
        return Err(ApiError::err_param("New password and confirm password do not match".to_string()));
    }

    let new_password = utils::hash::bcrypt(&req.new_password);

    let mut user: user::ActiveModel = user.into();
    user.password = Set(new_password);

    user.update(utils::db::conn()).await.map_err(|e| {
        tracing::error!(error = ?e, "Failed to update user password");
        ApiError::err_db()
    })?;

    Ok(ApiOk::new(()))
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