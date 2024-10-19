use axum::extract::Query;
use entity::user;
use sea_orm::{ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, QuerySelect};
use serde::{Deserialize, Serialize};
use utils::{request::{parse_page_request, PageRequest}, response::{ApiError, ApiOk, PageResponse, Result}};

#[derive(Debug, Deserialize)]
pub struct PageUserRequest {
    #[serde(flatten)]
    pub page_request: PageRequest,
    pub username: Option<String>,
    pub nickname: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct PageUserResponse {
    pub user_id: i64,
    pub username: String,
    pub nickname: Option<String>,
    pub avatar: Option<String>,
}

pub async fn page_user(
    Query(req): Query<PageUserRequest>
) -> Result<ApiOk<PageResponse<PageUserResponse>>> {
    let db = utils::db::conn();
    let mut query = user::Entity::find();

    if let Some(username) = &req.username {
        query = query.filter(user::Column::Username.contains(username));
    }

    if let Some(nickname) = &req.nickname {
        query = query.filter(user::Column::Nickname.contains(nickname));
    }

    let (offset, limit) = parse_page_request(req.page_request);

    let total = query.clone().count(db).await.map_err(|e| {
        tracing::error!(error = ?e, "Failed to count user");
        ApiError::err_db()
    })?;

    let users = query
        .order_by_asc(user::Column::UserId)
        .limit(limit)
        .offset(offset)
        .all(db)
        .await
        .map_err(|e| {
            tracing::error!(error = ?e, "Failed to query user");
            ApiError::err_db()
        })?;

    let resp = PageResponse {
        total,
        data: users.into_iter().map(|user| PageUserResponse {
            user_id: user.user_id,
            username: user.username,
            nickname: user.nickname,
            avatar: user.avatar,
        }).collect(),
    };

    Ok(ApiOk::new(resp))
}