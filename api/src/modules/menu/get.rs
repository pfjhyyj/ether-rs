use axum::extract::Path;
use sea_orm::EntityTrait;
use serde::Serialize;
use utils::response::{ApiOk, Result};

#[derive(Debug, Serialize)]
pub struct GetMenuDetailResponse {
    pub menu_id: i64,
    pub name: String,
    pub parent_id: Option<i64>,
    pub icon: Option<String>,
    pub menu_type: i32,
    pub sort: i32,
    pub path: Option<String>,
}

pub async fn get_menu(
    Path(menu_id): Path<i64>,
) -> Result<ApiOk<GetMenuDetailResponse>> {
    let menu = get_menu_by_id(menu_id).await?;
    let menu = GetMenuDetailResponse {
        menu_id: menu.menu_id,
        name: menu.name,
        parent_id: menu.parent_id,
        icon: menu.icon,
        menu_type: menu.menu_type,
        sort: menu.sort,
        path: menu.path,
    };
    Ok(ApiOk::new(menu))
}

async fn get_menu_by_id(id: i64) -> Result<entity::menu::Model> {
    let db = utils::db::conn();
    let menu = entity::menu::Entity::find_by_id(id)
        .one(db)
        .await
        .map_err(|e| {
            tracing::error!(error = ?e, "Failed to find menu");
            utils::response::ApiError::err_db()
        })?;

    if let Some(menu) = menu {
        Ok(menu)
    } else {
        Err(utils::response::ApiError::err_param("Menu not found".to_string()))
    }
}