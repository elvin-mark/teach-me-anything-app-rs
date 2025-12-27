use crate::{
    core::agents::GeneratedLesson,
    dto::lesson_dto::GenerateLessonRequest,
    errors::{app_error::AppError, types::ErrorResponse},
    middleware::auth::AuthGuard,
    services::lesson_service::LessonService,
};
use rocket::{Route, State, post, routes, serde::json::Json};

#[utoipa::path(
    post,
    path = "/api/lessons/generate",
    request_body = GenerateLessonRequest,
    params(
        ("Authorization" = String, Header, description = "Bearer token")
    ),
    responses(
        (status = 200, description = "Lesson generated", body = GeneratedLesson),
        (status = 400, description = "Invalid input", body = ErrorResponse)
    ),
    tag = "Lessons"
)]
#[post("/generate", data = "<request>")]
async fn generate_lesson(
    _auth: AuthGuard,
    request: Json<GenerateLessonRequest>,
    lesson_service: &State<LessonService>,
) -> Result<Json<GeneratedLesson>, AppError> {
    let lesson = lesson_service
        .generate_lesson(&request.topic)
        .await
        .map_err(|e| AppError::InternalError(format!("Failed to generate lesson: {}", e)))?;

    Ok(Json(lesson))
}

pub fn routes() -> Vec<Route> {
    routes![generate_lesson]
}
