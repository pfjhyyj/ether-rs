use sea_orm::{Set, ActiveModelTrait};
use serde::Deserialize;
use utils::{rejection::ValidatedJson, response::{ApiError, ApiOk, Result}};
use validator::Validate;


#[derive(Debug, Deserialize, Validate)]
pub struct CreatePermissionRequest {
    pub object: String,
    pub action: String,
    pub name: Option<String>,
    pub description: Option<String>,
}

pub async fn create_permission(
    ValidatedJson(req): ValidatedJson<CreatePermissionRequest>
) -> Result<ApiOk<(i64)>> {
    let new_permission = create_permission_by_request(req).await?;

    Ok(ApiOk::new(new_permission.permission_id))
}

async fn create_permission_by_request(req: CreatePermissionRequest) -> Result<entity::permission::Model> {
    let db = utils::db::conn();

    let new_permission = entity::permission::ActiveModel {
        object: Set(req.object),
        action: Set(req.action),
        name: Set(req.name),
        description: Set(req.description),
        ..Default::default()
    }.insert(db);

    let new_permission = new_permission.await.map_err(|e| {
        tracing::error!(error = ?e, "Failed to insert new permission");
        ApiError::err_db()
    })?;

    Ok(new_permission)
}