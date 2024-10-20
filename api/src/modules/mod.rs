use axum::{body::Body, http::Request, Router};
use tower_http::trace::TraceLayer;

pub mod auth;
pub mod user;
pub mod menu;
pub mod permission;
pub mod role;

pub fn get_router() -> Router {
    let open = Router::new().nest("/auth", auth::get_open_router());

    let auth = Router::new()
        .nest("/auth", auth::get_router())
        .nest("/users", user::get_router())
        .nest("/menus", menu::get_router())
        .layer(axum::middleware::from_fn(utils::middleware::jwt::handle));

    let modules = open.merge(auth);

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
