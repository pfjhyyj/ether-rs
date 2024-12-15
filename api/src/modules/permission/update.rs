use axum::extract::Path;
use sea_orm::{EntityTrait, Set, ActiveModelTrait};
use serde::Deserialize;
use utils::{rejection::ValidatedJson, response::{ApiError, ApiOk, Result}};
use validator::Validate;



#[derive(Debug, Deserialize, Validate)]
pub struct UpdatePermissionRequest {
    pub object: String,
    pub action: String,
    pub name: Option<String>,
    pub description: Option<String>,
}

pub async fn update_permission(
    Path(permission_id): Path<i64>,
    ValidatedJson(req): ValidatedJson<UpdatePermissionRequest>,
) -> Result<ApiOk<bool>> {
    let _ = update_permission_by_request(permission_id, req).await?;

    Ok(ApiOk::new(true))
}

async fn update_permission_by_request(permission_id: i64, req: UpdatePermissionRequest) -> Result<bool> {
    let db = utils::db::conn();
    let permission = entity::permission::Entity::find_by_id(permission_id)
        .one(db)
        .await
        .map_err(|e| {
            tracing::error!(error = ?e, "Failed to find permission");
            ApiError::err_db()
        })?;

    if permission.is_none() {
        return Err(ApiError::err_param("Permission not found".to_string()));
    }

    let mut permission: entity::permission::ActiveModel = permission.unwrap().into();
    permission.object = Set(req.object);
    permission.action = Set(req.action);
    permission.name = Set(req.name);
    permission.description = Set(req.description);

    permission.save(db)
        .await
        .map_err(|e| {
            tracing::error!(error = ?e, "Failed to update permission");
            ApiError::err_db()
        })?;

    Ok(true)
}