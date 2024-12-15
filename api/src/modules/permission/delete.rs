use axum::extract::Path;
use sea_orm::{EntityTrait, ModelTrait};
use utils::response::{ApiError, ApiOk, Result};


pub async fn delete_permission(
    Path(permission_id): Path<i64>,
) -> Result<ApiOk<bool>> {
    let _ = delete_permission_by_id(permission_id).await?;

    Ok(ApiOk::new(true))
}

async fn delete_permission_by_id(permission_id: i64) -> Result<bool> {
    let db = utils::db::conn();
    let permission = entity::permission::Entity::find_by_id(permission_id)
        .one(db)
        .await
        .map_err(|e| {
            tracing::error!(error = ?e, "Failed to find permission");
            ApiError::err_db()
        })?;
    
    if let Some(permission) = permission {
        permission.delete(db)
            .await
            .map_err(|e| {
                tracing::error!(error = ?e, "Failed to delete permission");
                ApiError::err_db()
            })?;
    }
    Ok(true)
}