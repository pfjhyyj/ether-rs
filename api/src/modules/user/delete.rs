use axum::extract::Path;
use entity::user;
use sea_orm::{EntityTrait, ModelTrait};
use utils::response::{ApiError, ApiOk, Result};


pub async fn delete_user(
    Path(user_id): Path<i64>,
) -> Result<ApiOk<bool>> {
    let _ = delete_user_by_id(user_id).await?;

    Ok(ApiOk::new(true))
}

async fn delete_user_by_id(user_id: i64) -> Result<bool> {
    let db = utils::db::conn();
    let user = user::Entity::find_by_id(user_id)
        .one(db)
        .await
        .map_err(|e| {
            tracing::error!(error = ?e, "Failed to find user");
            ApiError::err_db()
        })?;

    if let Some(user) = user {
        user.delete(db)
            .await
            .map_err(|e| {
                tracing::error!(error = ?e, "Failed to delete user");
                ApiError::err_db()
            })?;
    }

    Ok(true)
}