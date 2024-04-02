use axum::{routing::post, Router};

use self::login::login_by_username;

mod login;

pub fn get_router() -> Router {
  Router::new()
    .route("/login", post(login_by_username))
}