use axum::{body::Body, http::Request, routing::post, Router};
use tower_http::trace::TraceLayer;

use crate::controller;

pub fn init() -> Router {
  let auth = Router::new()
    .route("/login", post(controller::auth::login_by_username));
  
  let app = Router::new()
    .nest("/api", auth)
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