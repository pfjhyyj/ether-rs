use std::collections::HashMap;

use axum::{extract::Request, middleware::Next, response::Response};
use http::HeaderMap;


pub async fn handle(request: Request, next: Next) -> Response {
    let req_method = request.method().to_string();
    let req_uri = request.uri().to_string();
    // let req_header = header_to_string(request.headers());


    let response = next.run(request).await;

    tracing::info!(
        method = req_method,
        uri = req_uri,
        // headers = req_header,
        "processed request"
    );

    response
}

fn header_to_string(h: &HeaderMap) -> String {
  let mut map: HashMap<String, Vec<String>> = HashMap::new();

  for k in h.keys() {
      let mut vals: Vec<String> = Vec::new();
      for v in h.get_all(k) {
          if let Ok(s) = v.to_str() {
              vals.push(s.to_string())
          }
      }
      map.insert(k.to_string(), vals);
  }

  match serde_json::to_string(&map) {
      Ok(v) => v,
      Err(_) => String::from("<none>"),
  }
}