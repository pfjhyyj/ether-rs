use axum::extract::Path;
use sea_orm::{EntityTrait, ModelTrait};
use utils::response::{ApiError, ApiOk, Result};

pub async fn delete_role(
    Path(role_id): Path<i64>,
) -> Result<ApiOk<bool>> {
    let _ = delete_role_by_id(role_id).await?;

    Ok(ApiOk::new(true))
}

async fn delete_role_by_id(role_id: i64) -> Result<bool> {
    let db = utils::db::conn();
    let role = entity::role::Entity::find_by_id(role_id)
        .one(db)
        .await
        .map_err(|e| {
            tracing::error!(error = ?e, "Failed to find role");
            ApiError::err_db()
        })?;
    
    if let Some(role) = role {
        role.delete(db)
            .await
            .map_err(|e| {
                tracing::error!(error = ?e, "Failed to delete role");
                ApiError::err_db()
            })?;
    }
    Ok(true)
}