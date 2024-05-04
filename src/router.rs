use crate::modules;
use axum::{
    extract::{MatchedPath, Request},
    Router,
};
use tower_http::trace::TraceLayer;

pub fn get_router() -> Router {
    let router = Router::new().nest("/api", modules::set_router()).layer(
        TraceLayer::new_for_http()
            // Create our own span for the request and include the matched path. The matched
            // path is useful for figuring out which handler the request was routed to.
            .make_span_with(|req: &Request| {
                let method = req.method();
                let uri = req.uri();

                // axum automatically adds this extension.
                let matched_path = req
                    .extensions()
                    .get::<MatchedPath>()
                    .map(|matched_path| matched_path.as_str());

                tracing::debug_span!("request", %method, %uri, matched_path)
            })
            // By default `TraceLayer` will log 5xx responses but we're doing our specific
            // logging of errors so disable that
            .on_failure(()),
    );
    router
}
