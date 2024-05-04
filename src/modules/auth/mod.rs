use axum::{routing::post, Router};

pub mod register;

pub fn get_router() -> Router {
    Router::new()
        // .route("/login", post(login_by_username))
        // .route("/register", post(register_by_username))
        // .route("/logout", post(logout))
        .route("/register", post(register::register_by_username))   
}