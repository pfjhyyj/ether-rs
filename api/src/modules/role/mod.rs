use axum::{routing::{get, post}, Router};

mod create;
mod update;
mod delete;
mod get;
mod list;
mod add_permissions;
mod remove_permissions;
mod list_permissions;

pub fn get_router() -> Router {
    Router::new()
        .route("/", post(create::create_role).get(list::page_role))
        .route("/:role_id", get(get::get_role).put(update::update_role).delete(delete::delete_role))
        .route("/:role_id/permissions", get(list_permissions::page_role_permissions))
        .route("/:role_id/permissions/add", post(add_permissions::add_role_permissions))
        .route("/:role_id/permissions/remove", post(remove_permissions::remove_role_permissions))
}