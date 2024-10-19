use entity::user;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, Set, ActiveModelTrait};
use serde::Deserialize;
use utils::{rejection::ValidatedJson, response::{ApiError, ApiOk, Result}};
use validator::Validate;



#[derive(Debug, Deserialize, Validate)]
pub struct CreateUserRequest {
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

pub async fn create_user(
    ValidatedJson(req): ValidatedJson<CreateUserRequest>,
) -> Result<ApiOk<i64>> {
    let new_user = create_user_by_request(req).await?;

    Ok(ApiOk::new(new_user.user_id))
}

async fn create_user_by_request(req: CreateUserRequest) -> Result<user::Model> {
    let db = utils::db::conn();
    let user = user::Entity::find()
        .filter(user::Column::Username.eq(&req.username))
        .one(db)
        .await
        .map_err(|e| {
            tracing::error!(error = ?e, "Failed to query user by username");
            ApiError::err_db()
        })?;

    if user.is_some() {
        return Err(ApiError::err_param("Username already exists".to_string()));
    }

    let password = utils::hash::bcrypt(&req.password);

    let new_user = user::ActiveModel {
        username: Set(req.username),
        password: Set(password),
        email: Set(req.email),
        nickname: Set(req.nickname),
        ..Default::default()
    };

    let new_user = new_user.insert(db).await.map_err(|e| {
        tracing::error!(error = ?e, "Failed to insert new user");
        ApiError::err_db()
    })?;

    Ok(new_user)
}