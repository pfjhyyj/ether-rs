use axum::Router;

use crate::controller;


pub fn get_router() -> Router {
  let router = Router::new()
    .nest("/api", controller::set_router());
  router
}