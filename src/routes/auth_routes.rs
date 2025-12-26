use crate::{
    dto::auth_dto::{AuthResponse, LoginRequest, RegisterRequest},
    errors::app_error::AppError,
    services::auth_service::AuthService,
};
use rocket::{Route, State, post, routes, serde::json::Json};

#[post("/register", data = "<request>")]
async fn register(
    request: Json<RegisterRequest>,
    auth_service: &State<AuthService>,
) -> Result<Json<AuthResponse>, AppError> {
    let response = auth_service.register(request.into_inner()).await?;
    Ok(Json(response))
}

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
