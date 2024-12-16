use axum::extract::{Path, Query};
use sea_orm::{ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, QuerySelect};
use serde::{Deserialize, Serialize};
use utils::{request::{parse_page_request, PageRequest}, response::{ApiError, ApiOk, PageResponse, Result}};


#[derive(Debug, Deserialize)]
pub struct ListRolePermissionsRequest {
    #[serde(flatten)]
    pub page_request: PageRequest,
    pub object: Option<String>,
    pub action: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ListRolePermissionsResponse {
    pub permission_id: i64,
    pub object: String,
    pub action: String,
    pub name: Option<String>,
    pub description: Option<String>,
}

pub async fn page_role_permissions(
    Path(role_id): Path<i64>,
    Query(req): Query<ListRolePermissionsRequest>
) -> Result<ApiOk<PageResponse<ListRolePermissionsResponse>>> {
    let db = utils::db::conn();
    
    let mut query = entity::role_permission::Entity::find()
        .filter(entity::role_permission::Column::RoleId.eq(role_id));

    if let Some(object) = &req.object {
        query = query.filter(entity::permission::Entity, |permission| {
            permission.object.contains(object)
        });
    }

    if let Some(action) = &req.action {
        query = query.filter_related(entity::permission::Entity, |permission| {
            permission.action.contains(action)
        });
    }
        
    let (offset, limit) = parse_page_request(req.page_request);

    let total = query.clone().count(db).await.map_err(|e| {
        tracing::error!(error = ?e, "Failed to count role permission");
        ApiError::err_db()
    })?;

    let permissions: Vec<(entity::role_permission::Model, Option<entity::permission::Model>)> = query
        .order_by_asc(entity::permission::Column::PermissionId)
        .limit(limit)
        .offset(offset)
        .find_also_related(entity::permission::Entity)
        .all(db)
        .await
        .map_err(|e| {
            tracing::error!(error = ?e, "Failed to query role permission");
            ApiError::err_db()
        })?;

    let resp = PageResponse {
        total,
        data: permissions.into_iter().filter_map(|(role_permission, permission)| {
            permission.as_ref().map(|perm| ListRolePermissionsResponse {
                permission_id: role_permission.permission_id,
                object: perm.object.clone(),
                action: perm.action.clone(),
                name: perm.name.clone(),
                description: perm.description.clone(),
            })
        }).collect(),
    };

    Ok(ApiOk::new(resp))
}