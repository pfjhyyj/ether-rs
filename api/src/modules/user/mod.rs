use axum::{routing::get, Router};

pub mod current;

pub fn get_router() -> Router {
  Router::new()
    .route("/current", get(current::get_current_user))
}