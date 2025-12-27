use crate::{
    dto::user_dto::{CreateUserDto, UserResponseDto},
    errors::{app_error::AppError, types::ErrorResponse},
    middleware::auth::AuthGuard,
    services::user_service::UserService,
};
use rocket::{Route, State, delete, get, patch, routes, serde::json::Json};

#[utoipa::path(
    patch,
    path = "/api/users",
    request_body = CreateUserDto,
    params(
        ("Authorization" = String, Header, description = "Bearer token")
    ),
    responses(
        (status = 200, description = "Lesson generated", body = UserResponseDto),
        (status = 400, description = "Invalid input", body = ErrorResponse)
    ),
    tag = "Users"
)]
#[patch("/", data = "<update_user>")]
async fn update_user(
    _auth: AuthGuard,
    update_user: Json<CreateUserDto>,
    user_service: &State<UserService>,
) -> Result<Json<UserResponseDto>, AppError> {
    let user = user_service
        .update_user(_auth.user.id, update_user.into_inner())
        .await?;
    Ok(Json(user))
}

#[utoipa::path(
    delete,
    path = "/api/users",
    params(
        ("Authorization" = String, Header, description = "Bearer token")
    ),
    responses(
        (status = 200, description = "Lesson generated", body = String),
        (status = 400, description = "Invalid input", body = ErrorResponse)
    ),
    tag = "Users"
)]
#[delete("/")]
async fn delete_user(
    _auth: AuthGuard,
    user_service: &State<UserService>,
) -> Result<Json<String>, AppError> {
    let msg = user_service.delete_user(_auth.user.id).await?;
    Ok(Json(msg))
}

#[utoipa::path(
    get,
    path = "/api/users",
    params(
        ("Authorization" = String, Header, description = "Bearer token")
    ),
    responses(
        (status = 200, description = "Lesson generated", body = UserResponseDto),
        (status = 400, description = "Invalid input", body = ErrorResponse)
    ),
    tag = "Users"
)]
#[get("/")]
async fn get_user(
    _auth: AuthGuard,
    user_service: &State<UserService>,
) -> Result<Json<UserResponseDto>, AppError> {
    let user = user_service.get_user(_auth.user.id).await?;
    Ok(Json(user))
}

pub fn routes() -> Vec<Route> {
    routes![get_user, update_user, delete_user]
}
