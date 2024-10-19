use axum::{routing::{get, post, put}, Router};

pub mod login;
pub mod logout;
pub mod register;
pub mod current;
pub mod current_password;

pub fn get_open_router() -> Router {
    Router::new()
        .route("/login", post(login::login_by_username))
        .route("/register", post(register::register_by_username))
        .route("/current", get(current::get_current_user))
        .route("/current", put(current::update_current_user))
        .route("/current/password", put(current_password::update_password))
}

pub fn get_router() -> Router {
    Router::new().route("/logout", post(logout::logout))
}
