use entity::user;
use redis::Commands;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use utils::{
    rejection::ValidatedJson,
    response::{ApiError, ApiOk, Result},
};
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct LoginByUserNameRequest {
    #[validate(length(
        min = 6,
        max = 50,
        message = "Username must be between 6 and 50 characters"
    ))]
    pub username: String,
    #[validate(length(
        min = 6,
        max = 50,
        message = "Password must be between 6 and 50 characters"
    ))]
    pub password: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginByUserNameResponse {
    pub access_token: String,
    pub expire_time: String,
}

pub async fn login_by_username(
    ValidatedJson(req): ValidatedJson<LoginByUserNameRequest>,
) -> Result<ApiOk<LoginByUserNameResponse>> {
    let user = get_by_username(&req.username).await?;

    let is_valid = utils::hash::bcrypt_verify(&req.password, &user.password);
    if !is_valid {
        return Err(ApiError::err_param(
            "Invalid username or password".to_string(),
        ));
    }

    //  7 days to expire
    let expire_time = chrono::Utc::now().timestamp() + 60 * 60 * 24 * 7;
    let claims = utils::middleware::jwt::Claims {
        sub: user.user_id,
        exp: expire_time as usize,
    };
    let token = utils::jwt::generate_jwt_token(&claims).map_err(|e| {
        tracing::error!(error = ?e, "Failed to generate jwt token");
        ApiError::err_unknown("Failed to generate jwt token".to_string())
    })?;

    set_token_cache(&token, user.user_id)?;

    let resp = LoginByUserNameResponse {
        access_token: token,
        expire_time: expire_time.to_string(),
    };
    Ok(ApiOk::new(resp))
}

async fn get_by_username(username: &str) -> Result<user::Model> {
    let db = utils::db::conn();
    let user = user::Entity::find()
        .filter(user::Column::Username.eq(username))
        .one(db)
        .await
        .map_err(|e| {
            tracing::error!(error = ?e, "Failed to get user by username");
            ApiError::err_db("Failed to get user".to_string())
        })?
        .ok_or(ApiError::err_param(
            "Invalid username or password".to_string(),
        ))?;

    Ok(user)
}

fn set_token_cache(token: &str, user_id: i64) -> Result<()> {
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
    let _: () = conn.set(key, token).map_err(|e| {
        tracing::error!(error = ?e, "Failed to set token cache");
        ApiError::err_unknown("Failed to set token cache".to_string())
    })?;

    Ok(())
}
