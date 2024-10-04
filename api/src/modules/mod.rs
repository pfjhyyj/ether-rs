use axum::{body::Body, http::Request, Router};
use tower_http::trace::TraceLayer;

pub mod auth;
pub mod user;

pub fn get_router() -> Router {
  let modules = Router::new()
    .nest("/auth", auth::get_router());
  
  let app = Router::new()
    .nest("/api", modules)
    .layer(axum::middleware::from_fn(utils::middleware::cors::handle))
    .layer(axum::middleware::from_fn(utils::middleware::log::handle))
    .layer(
      TraceLayer::new_for_http().make_span_with(|request: &Request<Body>| {
        let req_id = match request
            .headers()
            .get("x-request-id")
            .and_then(|value| value.to_str().ok())
        {
            Some(v) => v.to_string(),
            None => String::from("unknown"),
        };
        tracing::error_span!("request_id", id = req_id)
    }),
    )
    .layer(axum::middleware::from_fn(utils::middleware::req_id::handle));

  app
}