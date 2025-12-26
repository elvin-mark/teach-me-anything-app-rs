use crate::{
    dto::auth_dto::{AuthResponse, LoginRequest, RegisterRequest},
    errors::{app_error::AppError, types::ErrorResponse},
    services::auth_service::AuthService,
};
use rocket::{Route, State, post, routes, serde::json::Json};

#[utoipa::path(
    post,
    path = "/api/auth/register",
    request_body = RegisterRequest,
    responses(
        (status = 200, description = "Register endpoint", body = AuthResponse),
        (status = 400, description = "Invalid input", body = ErrorResponse)
    ),
    tag = "Auth"
)]
#[post("/register", data = "<request>")]
async fn register(
    request: Json<RegisterRequest>,
    auth_service: &State<AuthService>,
) -> Result<Json<AuthResponse>, AppError> {
    let response = auth_service.register(request.into_inner()).await?;
    Ok(Json(response))
}

#[utoipa::path(
    post,
    path = "/api/auth/login",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login endpoint", body = AuthResponse),
        (status = 400, description = "Invalid input", body = ErrorResponse)
    ),
    tag = "Auth"
)]
#[post("/login", data = "<request>")]
async fn login(
    request: Json<LoginRequest>,
    auth_service: &State<AuthService>,
) -> Result<Json<AuthResponse>, AppError> {
    let response = auth_service.login(request.into_inner()).await?;
    Ok(Json(response))
}

pub fn routes() -> Vec<Route> {
    routes![register, login]
}
