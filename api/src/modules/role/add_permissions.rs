use axum::extract::Path;
use serde::Deserialize;
use utils::{rejection::ValidatedJson, response::{ApiError, ApiOk, Result}};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, Set};
use validator::Validate;


#[derive(Debug, Deserialize, Validate)]
pub struct AddRolePermissionsRequest {
    pub permission_ids: Vec<i64>,
}

pub async fn add_role_permissions(
    Path(role_id): Path<i64>,
    ValidatedJson(req): ValidatedJson<AddRolePermissionsRequest>
) -> Result<ApiOk<bool>> {
    let _ = add_role_permissions_by_request(role_id, req).await?;
    
    Ok(ApiOk::new(true))
}

async fn add_role_permissions_by_request(role_id: i64, req: AddRolePermissionsRequest) -> Result<bool> {
    let db = utils::db::conn();

    let role = entity::role::Entity::find_by_id(role_id)
        .one(db)
        .await
        .map_err(|e| {
            tracing::error!(error = ?e, "Failed to find role");
            ApiError::err_db()
        })?;
    
    if role.is_none() {
        return Err(ApiError::err_param("Role not found".to_string()));
    }

    // check if permissions exist
    let permissions = entity::permission::Entity::find()
        .filter(entity::permission::Column::PermissionId.is_in(req.permission_ids.clone()))
        .all(db)
        .await
        .map_err(|e| {
            tracing::error!(error = ?e, "Failed to find permissions");
            ApiError::err_db()
        })?;
    
    if permissions.len() != req.permission_ids.len() {
        return Err(ApiError::err_param("Permission not found".to_string()));
    }

    let role_permissions = entity::role_permission::Entity::find()
        .filter(entity::role_permission::Column::RoleId.eq(role_id))
        .all(db)
        .await
        .map_err(|e| {
            tracing::error!(error = ?e, "Failed to find role permissions");
            ApiError::err_db()
        })?;
    
    // check if permissions already exist
    let role_permission_ids: Vec<i64> = role_permissions.iter().map(|rp| rp.permission_id).collect();
    // get new permissions
    let new_permissions: Vec<i64> = req.permission_ids.iter().filter(|p| !role_permission_ids.contains(p)).map(|p| *p).collect();
    
    if new_permissions.is_empty() {
        return Err(ApiError::err_param("Permissions already exist".to_string()));
    }

    // insert new permissions
    let new_role_permissions = new_permissions.iter().map(|p| {
        entity::role_permission::ActiveModel {
            role_id: Set(role_id),
            permission_id: Set(*p),
            ..Default::default()
        }
    }).collect::<Vec<_>>();

    let _ = entity::role_permission::Entity::insert_many(new_role_permissions)
        .exec(db)
        .await
        .map_err(|e| {
            tracing::error!(error = ?e, "Failed to insert new role permissions");
            ApiError::err_db()
        })?;

    Ok(true)
}