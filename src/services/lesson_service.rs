use std::sync::Arc;

use crate::core::agents::{GeneratedLesson, LessonAgent};

#[derive(Clone)]
pub struct LessonService {
    lesson_agent: Arc<LessonAgent>,
}

impl LessonService {
    pub fn new(lesson_agent: Arc<LessonAgent>) -> Self {
        Self { lesson_agent }
    }

    pub async fn generate_lesson(
        &self,
        topic: &str,
    ) -> Result<GeneratedLesson, Box<dyn std::error::Error>> {
        self.lesson_agent.generate_lesson(topic).await
    }
}
