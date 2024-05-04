use axum::Router;

pub mod auth;

pub fn set_router() -> Router {
  let router = Router::new().nest("/auth", auth::get_router());
  router
}