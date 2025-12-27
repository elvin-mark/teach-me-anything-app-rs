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
