use axum::Json;
use entity::user;
use sea_orm::{EntityTrait, QueryFilter, ColumnTrait};
use serde::{Deserialize, Serialize};
use utils::{rejection::ValidatedJson, response::{ApiError, ApiOk, Result}};
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
pub struct LoginByUserNameResponse {
    pub access_token: String,
    pub expire_time: String,
}

pub async fn login_by_username(
  ValidatedJson(req): ValidatedJson<LoginByUserNameRequest>,
) -> Result<ApiOk<LoginByUserNameResponse>> {
    if let Err(e) = req.validate() {
        return Err(ApiError::err_param(e.to_string()))
    }

    let resp = LoginByUserNameResponse {
        access_token: "123".to_string(),
        expire_time: "123".to_string(),
    };

    Ok(ApiOk::new(resp))
}

// async fn _login_by_username(username: &str) {
//     // get user by username
//     let db = utils::db::conn();
//     let user = user::Entity::find()
//         .filter(user::Column::Username.eq(username))
//         .one(db)
//         .await
//         .map_err(|e| {
//             tracing::error!(error = ?e, "Failed to get user by username");
//             ApiError::err_param("Invalid username or password".to_string())
//         })?
//         .ok_or(ApiError::err_param("Invalid username or password".to_string()));


// }
