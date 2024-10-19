use axum::{routing::get, Router};

pub mod list;

pub fn get_router() -> Router {
    Router::new()
        .route("/", get(list::page_user))
}
