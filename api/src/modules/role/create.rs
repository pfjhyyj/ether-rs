use sea_orm::{Set, ActiveModelTrait};
use serde::Deserialize;
use utils::{rejection::ValidatedJson, response::{ApiError, ApiOk, Result}};
use validator::Validate;



#[derive(Debug, Deserialize, Validate)]
pub struct CreateRoleRequest {
    pub code: String,
    pub reference_type: Option<String>,
    pub reference_id: Option<i64>,
    pub name: String,
    pub description: Option<String>,
}

pub async fn create_role(
    ValidatedJson(req): ValidatedJson<CreateRoleRequest>
) -> Result<ApiOk<i64>> {
    let new_role = create_role_by_request(req).await?;

    Ok(ApiOk::new(new_role.role_id))
}

async fn create_role_by_request(req: CreateRoleRequest) -> Result<entity::role::Model> {
    let db = utils::db::conn();

    let new_role = entity::role::ActiveModel {
        code: Set(req.code),
        reference_type: Set(req.reference_type),
        reference_id: Set(req.reference_id),
        name: Set(req.name),
        description: Set(req.description),
        ..Default::default()
    }.insert(db);

    let new_role = new_role.await.map_err(|e| {
        tracing::error!(error = ?e, "Failed to insert new role");
        ApiError::err_db()
    })?;

    Ok(new_role)
}