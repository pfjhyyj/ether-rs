use axum::{routing::{get, post}, Router};

use super::menu::list::list_menu;

pub mod create;
pub mod update;
pub mod delete;
pub mod get;
pub mod list;

pub fn get_router() -> Router {
    Router::new()
        .route("/", post(create::create_role).get(list_menu))
        .route("/:role_id", get(get::get_role).put(update::update_role).delete(delete::delete_role))
}