use axum::{extract::Request, middleware::Next, response::Response};

pub async fn handle(request: Request, next: Next) -> Response {
    let req_method = request.method().to_string();
    let req_uri = request.uri().to_string();

    let response = next.run(request).await;

    tracing::info!(
        method = req_method,
        uri = req_uri,
        "processed request"
    );

    response
}