use axum::{routing::post, Router};

pub mod login;
pub mod logout;
pub mod register;

pub fn get_open_router() -> Router {
  Router::new()
    .route("/login", post(login::login_by_username))
    .route("/register", post(register::register_by_username))
}


pub fn get_router() -> Router {
  Router::new()
    .route("/logout", post(logout::logout))
}