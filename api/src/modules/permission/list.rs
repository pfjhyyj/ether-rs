use axum::extract::Query;
use sea_orm::{ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, QuerySelect};
use serde::{Deserialize, Serialize};
use utils::{request::{parse_page_request, PageRequest}, response::{ApiError, ApiOk, PageResponse, Result}};

#[derive(Debug, Deserialize)]
pub struct PagePermssionRequest {
    #[serde(flatten)]
    pub page_request: PageRequest,
    pub object: Option<String>,
    pub action: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct PagePermissionResponse {
    pub permission_id: i64,
    pub object: String,
    pub action: String,
    pub name: Option<String>,
    pub description: Option<String>,
}

pub async fn page_permission(
    Query(req): Query<PagePermssionRequest>
) -> Result<ApiOk<PageResponse<PagePermissionResponse>>> {
    let db = utils::db::conn();
    let mut query = entity::permission::Entity::find();

    if let Some(object) = &req.object {
        query = query.filter(entity::permission::Column::Object.contains(object));
    }

    if let Some(action) = &req.action {
        query = query.filter(entity::permission::Column::Action.contains(action));
    }

    let (offset, limit) = parse_page_request(req.page_request);

    let total = query.clone().count(db).await.map_err(|e| {
        tracing::error!(error = ?e, "Failed to count permission");
        ApiError::err_db()
    })?;

    let permissions = query
        .order_by_asc(entity::permission::Column::PermissionId)
        .limit(limit)
        .offset(offset)
        .all(db)
        .await
        .map_err(|e| {
            tracing::error!(error = ?e, "Failed to query permission");
            ApiError::err_db()
        })?;

    let resp = PageResponse {
        total,
        data: permissions.into_iter().map(|permission| PagePermissionResponse {
            permission_id: permission.permission_id,
            object: permission.object,
            action: permission.action,
            name: permission.name,
            description: permission.description,
        }).collect(),
    };

    Ok(ApiOk::new(resp))
}