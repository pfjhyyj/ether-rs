use axum::{routing::get, Router};

pub mod list;
pub mod create;
pub mod update;
pub mod delete;
pub mod get;

pub fn get_router() -> Router {
    Router::new()
        .route("/", get(list::page_user).post(create::create_user))
        .route("/:user_id", get(get::get_user).put(update::update_user).delete(delete::delete_user))
}
