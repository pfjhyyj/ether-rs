use axum::extract::Path;
use entity::user;
use sea_orm::{EntityTrait, Set, ActiveModelTrait};
use serde::Deserialize;
use utils::{rejection::ValidatedJson, response::{ApiError, ApiOk, Result}};
use validator::Validate;



#[derive(Debug, Deserialize, Validate)]
pub struct UpdateUserRequest {
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
    pub email: Option<String>,
    pub nickname: Option<String>,
}

pub async fn update_user(
    Path(user_id): Path<i64>,
    ValidatedJson(req): ValidatedJson<UpdateUserRequest>,
) -> Result<ApiOk<bool>> {
    let _ = update_user_by_request(user_id, req).await?;

    Ok(ApiOk::new(true))
}

async fn update_user_by_request(user_id: i64, req: UpdateUserRequest) -> Result<bool> {
    let db = utils::db::conn();
    let user = user::Entity::find_by_id(user_id)
        .one(db)
        .await
        .map_err(|e| {
            tracing::error!(error = ?e, "Failed to find user");
            ApiError::err_db()
        })?;

    if user.is_none() {
        return Err(ApiError::err_param("User not found".to_string()));
    }

    let mut user: user::ActiveModel = user.unwrap().into();
    user.username = Set(req.username);
    user.password = Set(utils::hash::bcrypt(&req.password));
    user.email = Set(req.email);
    user.nickname = Set(req.nickname);

    user.save(db)
        .await
        .map_err(|e| {
            tracing::error!(error = ?e, "Failed to update user");
            ApiError::err_db()
        })?;

    Ok(true)
}