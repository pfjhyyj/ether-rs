use axum::extract::Path;
use sea_orm::{EntityTrait, Set, ActiveModelTrait};
use serde::Deserialize;
use utils::{rejection::ValidatedJson, response::{ApiError, ApiOk, Result}};
use validator::Validate;



#[derive(Debug, Deserialize, Validate)]
pub struct UpdateMenuRequest {
    pub name: String,
    pub parent_id: Option<i64>,
    pub icon: Option<String>,
    pub menu_type: i32,
    pub sort: i32,
    pub path: Option<String>,
}

pub async fn update_menu(
    Path(menu_id): Path<i64>,
    ValidatedJson(req): ValidatedJson<UpdateMenuRequest>,
) -> Result<ApiOk<bool>> {
    let _ = update_menu_by_request(menu_id, req).await?;

    Ok(ApiOk::new(true))
}

async fn update_menu_by_request(menu_id: i64, req: UpdateMenuRequest) -> Result<bool> {
    let db = utils::db::conn();
    let menu = entity::menu::Entity::find_by_id(menu_id)
        .one(db)
        .await
        .map_err(|e| {
            tracing::error!(error = ?e, "Failed to find menu");
            ApiError::err_db()
        })?;

    if menu.is_none() {
        return Err(ApiError::err_param("Menu not found".to_string()));
    }

    let mut menu: entity::menu::ActiveModel = menu.unwrap().into();
    menu.name = Set(req.name);
    menu.parent_id = Set(req.parent_id);
    menu.icon = Set(req.icon);
    menu.menu_type = Set(req.menu_type);
    menu.sort = Set(req.sort);
    menu.path = Set(req.path);


    menu.save(db)
        .await
        .map_err(|e| {
            tracing::error!(error = ?e, "Failed to update menu");
            ApiError::err_db()
        })?;

    Ok(true)
}