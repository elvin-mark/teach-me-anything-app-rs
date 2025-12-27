use crate::{
    core::agents::types::GeneratedRoadmap,
    dto::planner_dto::GenerateRoadmapRequest,
    errors::{app_error::AppError, types::ErrorResponse},
    middleware::auth::AuthGuard,
    services::planner_service::PlannerService,
};
use rocket::{Route, State, post, routes, serde::json::Json};

#[utoipa::path(
    post,
    path = "/api/planner/roadmap/generate",
    request_body = GenerateRoadmapRequest,
    params(
        ("Authorization" = String, Header, description = "Bearer token")
    ),
    responses(
        (status = 200, description = "Lesson generated", body = GeneratedRoadmap),
        (status = 400, description = "Invalid input", body = ErrorResponse)
    ),
    tag = "Planner"
)]
#[post("/roadmap/generate", data = "<request>")]
async fn generate_roadmap(
    _auth: AuthGuard,
    request: Json<GenerateRoadmapRequest>,
    planner_service: &State<PlannerService>,
) -> Result<Json<GeneratedRoadmap>, AppError> {
    let roadmap = planner_service
        .generate_roadmap(&request.goal)
        .await
        .map_err(|e| AppError::InternalError(format!("Failed to generate lesson: {}", e)))?;

    Ok(Json(roadmap))
}

pub fn routes() -> Vec<Route> {
    routes![generate_roadmap]
}
