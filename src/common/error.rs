use axum::extract::rejection::{FormRejection, JsonRejection};
use sea_orm::DbErr;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ServerError {
    #[error(transparent)]
    ValidationError(#[from] validator::ValidationErrors),

    #[error(transparent)]
    AxumFormRejection(#[from] FormRejection),

    #[error(transparent)]
    AxumJsonRejection(#[from] JsonRejection),

    #[error(transparent)]
    DatabaseError(#[from] DbErr),

    #[error("System error: {0}")]
    SystemError(String),
}
