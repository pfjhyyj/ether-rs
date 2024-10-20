use std::collections::HashMap;

use axum::Extension;
use entity::menu;
use sea_orm::EntityTrait;
use serde::Serialize;
use serde_json::Value;
use utils::{middleware::jwt::Claims, response::{ApiOk, Result}};

#[derive(Debug, Serialize, Clone)]
pub struct MenuResponse {
    pub menu_id: i64,
    pub parent_id: Option<i64>,
    pub name: String,
    pub menu_type: i32,
    pub icon: Option<String>,
    pub path: Option<String>,
    pub sort: i32,
    pub extra: Option<Value>,
    pub children: Vec<MenuResponse>,
}

#[derive(Debug, Serialize)]
pub struct ListMenuResponse {
    pub menus: Vec<MenuResponse>,
}

pub async fn list_menu(
    Extension(token_data): Extension<Claims>
) -> Result<ApiOk<ListMenuResponse>> {
    let menus = get_menu_list().await?;
    let menus = build_menu_forest(menus);
    let menus = ListMenuResponse { menus };
    Ok(ApiOk::new(menus))
}

async fn get_menu_list() -> Result<Vec<menu::Model>> {
    let db = utils::db::conn();
    let menus = entity::menu::Entity::find()
        .all(db)
        .await
        .map_err(|e| {
            tracing::error!(error = ?e, "Failed to find menus");
            utils::response::ApiError::err_db()
        })?;
    
    Ok(menus)
}

fn build_menu_forest(menus: Vec<menu::Model>) -> Vec<MenuResponse> {
    let mut menu_map: HashMap<Option<i64>, Vec<MenuResponse>> = HashMap::new();
    
    for menu in menus {
        let menu = MenuResponse {
            menu_id: menu.menu_id,
            parent_id: menu.parent_id,
            name: menu.name,
            menu_type: menu.menu_type,
            icon: menu.icon,
            path: menu.path,
            sort: menu.sort,
            extra: menu.extra,
            children: Vec::new(),
        };
        menu_map.entry(menu.parent_id).or_default().push(menu);
    }

    fn attach_children(parent_id: Option<i64>, menu_map: &mut HashMap<Option<i64>, Vec<MenuResponse>>) -> Vec<MenuResponse> {
        if let Some(children) = menu_map.remove(&parent_id) {
            children
                .into_iter()
                .map(|mut menu| {
                    menu.children = attach_children(Some(menu.menu_id), menu_map);
                    menu
                })
                .collect()
        } else {
            Vec::new()
        }
    }

    attach_children(None, &mut menu_map)
}