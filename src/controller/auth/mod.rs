use axum::{routing::post, Router};

use self::{login::login_by_username, logout::logout, register::register_by_username};

mod login;
mod logout;
mod register;

pub fn get_router() -> Router {
  Router::new()
    .route("/login", post(login_by_username))
    .route("/register", post(register_by_username))
    .route("/logout", post(logout))
}