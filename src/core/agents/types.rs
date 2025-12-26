use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratedLesson {
    pub title: String,
    pub content: String,
}
