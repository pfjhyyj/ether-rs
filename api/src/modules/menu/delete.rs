use axum::extract::Path;
use entity::menu;
use sea_orm::{EntityTrait, ModelTrait};
use utils::response::{ApiError, ApiOk, Result};



pub async fn delete_menu(
    Path(menu_id): Path<i64>,
) -> Result<ApiOk<bool>> {
    let _ = delete_menu_by_id(menu_id).await?;

    Ok(ApiOk::new(true))
}


async fn delete_menu_by_id(menu_id: i64) -> Result<bool> {
    let db = utils::db::conn();
    let menu = menu::Entity::find_by_id(menu_id)
        .one(db)
        .await
        .map_err(|e| {
            tracing::error!(error = ?e, "Failed to find menu");
            ApiError::err_db()
        })?;
    
    if let Some(menu) = menu {
        menu.delete(db)
            .await
            .map_err(|e| {
                tracing::error!(error = ?e, "Failed to delete menu");
                ApiError::err_db()
            })?;
    }
    Ok(true)
}