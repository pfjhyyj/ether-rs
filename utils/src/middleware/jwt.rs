use ::redis::Commands;
use axum::{
    extract::Request,
    middleware::Next,
    response::{IntoResponse, Response},
};
use http::header::AUTHORIZATION;
use serde::{Deserialize, Serialize};

use crate::{jwt::verify_jwt_token, response::ApiError};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    // sub: subject, user id
    pub sub: i64,
    // exp: expiration time
    pub exp: usize,
}

pub async fn handle(mut request: Request, next: Next) -> Response {
    let token = request.headers().get(AUTHORIZATION);
    let token_data = match token {
        None => Claims { sub: 0, exp: 0 },
        Some(token) => match token.to_str() {
            Ok(token) => {
                let token = token.replace("Bearer ", "");
                let token_data = verify_jwt_token::<Claims>(&token);
                match token_data {
                    Ok(data) => data,
                    Err(_) => Claims { sub: 0, exp: 0 },
                }
            }
            Err(_) => Claims { sub: 0, exp: 0 },
        },
    };
    if token_data.exp == 0 {
        return ApiError::err_unauthenticated("Token invalid".to_string()).into_response();
    }

    let mut conn = match crate::redis::redis_pool().get() {
        Ok(c) => c,
        Err(e) => {
            tracing::error!(error = ?e, "Failed to get redis connection");
            return ApiError::err_unknown("Failed to get redis connection".to_string())
                .into_response();
        }
    };
    let key = format!("token:{}", token_data.sub);
    let token_result: Result<String, ApiError> = conn
        .get(key)
        .map_err(|_| ApiError::err_unauthenticated("Token invalid".to_string()));
    match token_result {
        Ok(token) => {
            if token.is_empty() {
                return ApiError::err_unauthenticated("Token invalid".to_string()).into_response();
            }
        }
        Err(e) => return e.into_response(),
    }

    request.extensions_mut().insert(token_data);
    next.run(request).await
}
