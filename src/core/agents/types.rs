use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct GeneratedLesson {
    pub title: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct GeneratedRoadmap {
    pub goal: String,
    pub milestones: Box<[String]>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct GeneratedQuestion {
    pub question: String,
    pub hint: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct GeneratedQuestions {
    pub questions: Box<[GeneratedQuestion]>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct GeneratedGrading {
    pub score: i32,
    pub feedback: String,
}
