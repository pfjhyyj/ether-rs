use axum::Extension;
use redis::Commands;
use utils::{
    middleware::jwt::Claims,
    response::{ApiError, ApiOk, Result},
};

pub async fn logout(Extension(token_data): Extension<Claims>) -> Result<ApiOk<()>> {
    clear_token_cache(token_data.sub)?;
    Ok(ApiOk::new(()))
}

fn clear_token_cache(user_id: i64) -> Result<()> {
    let mut conn = match utils::redis::redis_pool().get() {
        Ok(c) => c,
        Err(e) => {
            tracing::error!(error = ?e, "Failed to get redis connection");
            return Err(ApiError::err_unknown(
                "Failed to get redis connection".to_string(),
            ));
        }
    };

    let key = format!("token:{}", user_id);
    let _: () = conn.del(key).map_err(|e| {
        tracing::error!(error = ?e, "Failed to clear token cache");
        ApiError::err_unknown("Failed to clear token cache".to_string())
    })?;

    Ok(())
}
