use axum::{routing::{get, put}, Router};

pub mod current;

pub fn get_router() -> Router {
    Router::new()
        .route("/current", get(current::get_current_user))
        .route("/current", put(current::update_current_user))
        .route("/current/password", put(current::update_password))
}
