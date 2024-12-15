use axum::extract::Path;
use sea_orm::EntityTrait;
use serde::Serialize;
use utils::response::{ApiOk, Result};

#[derive(Debug, Serialize)]
pub struct GetRoleDetailResponse {
    pub role_id: i64,
    pub code: String,
    pub reference_type: Option<String>,
    pub reference_id: Option<i64>,
    pub name: String,
    pub description: Option<String>,
}

pub async fn get_role(
    Path(role_id): Path<i64>,
) -> Result<ApiOk<GetRoleDetailResponse>> {
    let role = get_role_by_id(role_id).await?;
    let role = GetRoleDetailResponse {
        role_id: role.role_id,
        code: role.code,
        reference_type: role.reference_type,
        reference_id: role.reference_id,
        name: role.name,
        description: role.description,
    };
    Ok(ApiOk::new(role))
}

async fn get_role_by_id(id: i64) -> Result<entity::role::Model> {
    let db = utils::db::conn();
    let role = entity::role::Entity::find_by_id(id)
        .one(db)
        .await
        .map_err(|e| {
            tracing::error!(error = ?e, "Failed to find role");
            utils::response::ApiError::err_db()
        })?;

    if let Some(role) = role {
        Ok(role)
    } else {
        Err(utils::response::ApiError::err_param("Role not found".to_string()))
    }
}