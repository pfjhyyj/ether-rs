use axum::{extract::Request, middleware::Next, response::Response};
use http::header::AUTHORIZATION;
use serde::{Deserialize, Serialize};

use crate::jwt::verify_jwt_token;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Claims {
  // sub: subject, user id
  sub: String,
  // exp: expiration time
  exp: usize,
}


pub async fn handle(mut request: Request, next: Next) -> Response {
  let token = request.headers().get(AUTHORIZATION);
  let token_data = match token {
    None => Claims { sub: String::new(), exp: 0 },
    Some(token) => match token.to_str() {
      Ok(token) => {
        let token = token.replace("Bearer ", "");
        let token_data = verify_jwt_token::<Claims>(&token);
        match token_data {
          Ok(data) => data,
          Err(_) => Claims { sub: String::new(), exp: 0 },
        }
      }
      Err(_) => Claims { sub: String::new(), exp: 0 },
    },
  };
  request.extensions_mut().insert(token_data);
  next.run(request).await
}