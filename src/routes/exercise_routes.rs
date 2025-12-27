use crate::{
    core::agents::types::{GeneratedGrading, GeneratedQuestions},
    dto::exercise_dto::{GenerateQuestionsRequest, GradeQuestionRequest},
    errors::{app_error::AppError, types::ErrorResponse},
    middleware::auth::AuthGuard,
    services::exercise_service::ExerciseService,
};
use rocket::{Route, State, post, routes, serde::json::Json};

#[utoipa::path(
    post,
    path = "/api/exercise/questions/generate",
    request_body = GenerateQuestionsRequest,
    params(
        ("Authorization" = String, Header, description = "Bearer token")
    ),
    responses(
        (status = 200, description = "Lesson generated", body = GeneratedQuestions),
        (status = 400, description = "Invalid input", body = ErrorResponse)
    ),
    tag = "Exercise"
)]
#[post("/questions/generate", data = "<request>")]
async fn generate_questions(
    _auth: AuthGuard,
    request: Json<GenerateQuestionsRequest>,
    exercise_service: &State<ExerciseService>,
) -> Result<Json<GeneratedQuestions>, AppError> {
    let questions = exercise_service
        .generate_questions(&request.content)
        .await
        .map_err(|e| AppError::InternalError(format!("Failed to generate lesson: {}", e)))?;

    Ok(Json(questions))
}

#[utoipa::path(
    post,
    path = "/api/exercise/question/grade",
    request_body = GradeQuestionRequest,
    params(
        ("Authorization" = String, Header, description = "Bearer token")
    ),
    responses(
        (status = 200, description = "Question graded", body = GeneratedGrading),
        (status = 400, description = "Invalid input", body = ErrorResponse)
    ),
    tag = "Exercise"
)]
#[post("/question/grade", data = "<request>")]
async fn grade_question(
    _auth: AuthGuard,
    request: Json<GradeQuestionRequest>,
    exercise_service: &State<ExerciseService>,
) -> Result<Json<GeneratedGrading>, AppError> {
    let questions = exercise_service
        .grade_question(&request.question, &request.answer)
        .await
        .map_err(|e| AppError::InternalError(format!("Failed to generate lesson: {}", e)))?;

    Ok(Json(questions))
}

pub fn routes() -> Vec<Route> {
    routes![generate_questions, grade_question]
}
