use axum::{routing::{get, post}, Router};


pub mod create;
pub mod update;
pub mod delete;
pub mod get;
pub mod list;

pub fn get_router() -> Router {
    Router::new()
        .route("/", post(create::create_menu).get(list::list_menu))
        .route("/:menu_id", get(get::get_menu).put(update::update_menu).delete(delete::delete_menu))
}