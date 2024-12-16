use axum::extract::Path;
use serde::Deserialize;
use utils::{rejection::ValidatedJson, response::{ApiError, ApiOk, Result}};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use validator::Validate;


#[derive(Deserialize, Validate)]
pub struct RemoveRolePermissionsRequest {
    pub permission_ids: Vec<i64>,
}

pub async fn remove_role_permissions(
    Path(role_id): Path<i64>,
    ValidatedJson(req): ValidatedJson<RemoveRolePermissionsRequest>
) -> Result<ApiOk<bool>> {
    let _ = remove_role_permissions_by_request(role_id, req).await?;
    Ok(ApiOk::new(true))
}

async fn remove_role_permissions_by_request(role_id: i64, req: RemoveRolePermissionsRequest) -> Result<bool> {
    let db = utils::db::conn();

    let _ = entity::role_permission::Entity::delete_many()
        .filter(entity::role_permission::Column::RoleId.eq(role_id))
        .filter(entity::role_permission::Column::PermissionId.is_in(req.permission_ids.clone()))
        .exec(db)
        .await
        .map_err(|e| {
            tracing::error!(error = ?e, "Failed to delete role permissions");
            ApiError::err_db()
        })?;

    Ok(true)
}