use axum::Router;

use crate::modules;


pub fn get_router() -> Router {
  let router = Router::new()
    .nest("/api", modules::set_router());
  router
}