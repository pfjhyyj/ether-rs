use chrono::Utc;
use entity::user;
use jsonwebtoken::{encode, EncodingKey, Header};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::common::{
    error::ServerError,
    response::{ApiResponse, ApiResponseCode},
    validate::ValidatedJson,
};

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

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

pub async fn login_by_username(
    ValidatedJson(req): ValidatedJson<LoginByUserNameRequest>,
) -> Result<ApiResponse<LoginByUserNameResponse>, ServerError> {
    let user = get_user_by_username(&req.username).await?;

    let is_valid = check_user_password(&user, &req.password);
    if !is_valid {
        return Err(ServerError::SystemError(
            "Invalid username or password".to_string(),
        ));
    }

    // set the expire time
    let expire_time = Utc::now().timestamp() + 3600 * 24 * 7;
    let token = generate_jwt_token(&user.user_id, &expire_time);

    let resp = LoginByUserNameResponse {
        access_token: token,
        expire_time: expire_time.to_string(),
    };

    Ok(ApiResponse {
        code: ApiResponseCode::Ok as i32,
        msg: None,
        data: Some(resp),
    })
}

async fn get_user_by_username(username: &str) -> Result<user::Model, ServerError> {
    let db = client::database::get_db_connection().await;
    let user = user::Entity::find()
        .filter(user::Column::Username.contains(username))
        .one(db)
        .await?;
    if let Some(user) = user {
        Ok(user)
    } else {
        Err(ServerError::SystemError(
            "Invalid username or password".to_string(),
        ))
    }
}



fn check_user_password(user: &user::Model, password: &str) -> bool {
    let is_valid = bcrypt::verify(password, &user.password).unwrap();
    is_valid
}

fn generate_jwt_token(user_id: &i64, expire_time: &i64) -> String {
    let claims = Claims {
        sub: user_id.to_string(),
        exp: *expire_time as usize,
    };

    let jwt_secret = utils::env::get_env::<String>("JWT_SECRET");
    let key = EncodingKey::from_secret(jwt_secret.as_bytes());
    let token = encode(&Header::default(), &claims, &key);

    token.unwrap()
}