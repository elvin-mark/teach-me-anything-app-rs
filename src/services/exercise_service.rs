use std::sync::Arc;

use crate::core::agents::{
    exercise_agent::ExerciseAgent,
    types::{GeneratedGrading, GeneratedQuestions},
};

#[derive(Clone)]
pub struct ExerciseService {
    exercise_agent: Arc<ExerciseAgent>,
}

impl ExerciseService {
    pub fn new(exercise_agent: Arc<ExerciseAgent>) -> Self {
        Self { exercise_agent }
    }

    pub async fn generate_questions(
        &self,
        topic: &str,
    ) -> Result<GeneratedQuestions, Box<dyn std::error::Error>> {
        self.exercise_agent.generate_questions(topic).await
    }

    pub async fn grade_question(
        &self,
        question: &str,
        answer: &str,
    ) -> Result<GeneratedGrading, Box<dyn std::error::Error>> {
        self.exercise_agent.grade_question(question, answer).await
    }
}
