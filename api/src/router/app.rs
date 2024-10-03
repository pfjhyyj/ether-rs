use axum::{routing::post, Router};

use crate::controller;

pub fn init() -> Router {
  let auth = Router::new()
    .route("/login", post(controller::auth::login_by_username));
  
  let app = Router::new()
    .nest("/api", auth);
  app
}