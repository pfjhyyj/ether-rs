use axum::extract::Path;
use sea_orm::{EntityTrait, Set, ActiveModelTrait};
use serde::Deserialize;
use utils::{rejection::ValidatedJson, response::{ApiError, ApiOk, Result}};
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateRoleRequest {
    pub code: String,
    pub reference_type: Option<String>,
    pub reference_id: Option<i64>,
    pub name: String,
    pub description: Option<String>,
}

pub async fn update_role(
    Path(role_id): Path<i64>,
    ValidatedJson(req): ValidatedJson<UpdateRoleRequest>,
) -> Result<ApiOk<bool>> {
    let _ = update_role_by_request(role_id, req).await?;

    Ok(ApiOk::new(true))
}

async fn update_role_by_request(role_id: i64, req: UpdateRoleRequest) -> Result<bool> {
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

    let mut role: entity::role::ActiveModel = role.unwrap().into();
    role.code = Set(req.code);
    role.reference_type = Set(req.reference_type);
    role.reference_id = Set(req.reference_id);
    role.name = Set(req.name);
    role.description = Set(req.description);

    role.save(db)
        .await
        .map_err(|e| {
            tracing::error!(error = ?e, "Failed to update role");
            ApiError::err_db()
        })?;

    Ok(true)
}