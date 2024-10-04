use axum::{routing::post, Router};

pub mod login;
pub mod register;
pub mod logout;

pub fn get_router() -> Router {
    Router::new()
      .route("/login", post(login::login_by_username))
}