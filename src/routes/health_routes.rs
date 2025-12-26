use crate::errors::types::ErrorResponse;
use crate::{dto::health_dto::HealthResponseDto, errors::app_error::AppError};
use rocket::serde::json::Json;
use rocket::{Route, get, routes};

#[utoipa::path(
    get,
    path = "/api/health",
    responses(
        (status = 200, description = "Health endpoint", body = String),
        (status = 400, description = "Invalid input", body = ErrorResponse)
    ),
    tag = "Health"
)]
#[get("/")]
pub async fn get_health() -> Result<Json<HealthResponseDto>, AppError> {
    Ok(Json(HealthResponseDto { status: "ok" }))
}

pub fn routes() -> Vec<Route> {
    routes![get_health]
}
