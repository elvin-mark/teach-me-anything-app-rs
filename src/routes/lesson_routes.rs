use crate::{
    core::agents::GeneratedLesson, errors::app_error::AppError, middleware::auth::AuthGuard,
    services::lesson_service::LessonService,
};
use rocket::{Route, State, post, routes, serde::json::Json};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GenerateLessonRequest {
    pub topic: String,
}

#[post("/generate", data = "<request>")]
async fn generate_lesson(
    auth_result: Result<AuthGuard, AppError>,
    request: Json<GenerateLessonRequest>,
    lesson_service: &State<LessonService>,
) -> Result<Json<GeneratedLesson>, AppError> {
    let _auth = auth_result?;
    let lesson = lesson_service
        .generate_lesson(&request.topic)
        .await
        .map_err(|e| AppError::InternalError(format!("Failed to generate lesson: {}", e)))?;

    Ok(Json(lesson))
}

pub fn routes() -> Vec<Route> {
    routes![generate_lesson]
}
