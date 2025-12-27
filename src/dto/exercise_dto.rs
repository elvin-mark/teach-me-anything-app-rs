use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
pub struct GenerateQuestionsRequest {
    pub content: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct GradeQuestionRequest {
    pub question: String,
    pub answer: String,
}
