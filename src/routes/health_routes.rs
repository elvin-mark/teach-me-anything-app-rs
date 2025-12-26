use crate::{dto::health_dto::HealthResponseDto, errors::app_error::AppError};
use rocket::serde::json::Json;
use rocket::{Route, get, routes};

#[get("/")]
pub async fn get_health() -> Result<Json<HealthResponseDto>, AppError> {
    Ok(Json(HealthResponseDto { status: "ok" }))
}
pub fn routes() -> Vec<Route> {
    routes![get_health]
}
