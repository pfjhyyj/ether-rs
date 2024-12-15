use axum::extract::Path;
use sea_orm::EntityTrait;
use serde::Serialize;
use utils::response::{ApiOk, Result};

#[derive(Debug, Serialize)]
pub struct GetPermssionDetailResponse {
    pub permission_id: i64,
    pub object: String,
    pub action: String,
    pub name: Option<String>,
    pub description: Option<String>,
}

pub async fn get_permission(
    Path(permission_id): Path<i64>,
) -> Result<ApiOk<GetPermssionDetailResponse>> {
    let permission = get_permission_by_id(permission_id).await?;
    let permission = GetPermssionDetailResponse {
        permission_id: permission.permission_id,
        object: permission.object,
        action: permission.action,
        name: permission.name,
        description: permission.description,
    };
    Ok(ApiOk::new(permission))
}

async fn get_permission_by_id(id: i64) -> Result<entity::permission::Model> {
    let db = utils::db::conn();
    let permission = entity::permission::Entity::find_by_id(id)
        .one(db)
        .await
        .map_err(|e| {
            tracing::error!(error = ?e, "Failed to find permission");
            utils::response::ApiError::err_db()
        })?;

    if let Some(permission) = permission {
        Ok(permission)
    } else {
        Err(utils::response::ApiError::err_param("Permission not found".to_string()))
    }
}