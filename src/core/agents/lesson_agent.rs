use super::types::GeneratedLesson;
use crate::core::llm::Llm;
use core::str;
use std::sync::Arc;

#[derive(Clone)]
pub struct LessonAgent {
    llm: Arc<dyn Llm>,
}

impl LessonAgent {
    pub fn new(llm: Arc<dyn Llm>) -> Arc<Self> {
        Arc::new(Self { llm })
    }

    pub async fn generate_lesson(
        &self,
        topic: &str,
    ) -> Result<GeneratedLesson, Box<dyn std::error::Error>> {
        let prompt = self.generate_lesson_generation_prompt(topic);

        let resp = self.llm.get_response(&prompt).await?;

        // Extract content from response
        let content = resp
            .choices
            .first()
            .map(|choice| &choice.message.content)
            .ok_or("No response from LLM")?;

        // Clean the response (remove markdown code blocks)
        let cleaned = self.clean_json_response(content);

        log::debug!("Response from LLM: {}", cleaned);

        // Parse JSON
        let generated_lesson: GeneratedLesson =
            serde_json::from_str(&cleaned).map_err(|e| format!("Failed to parse JSON: {}", e))?;

        Ok(generated_lesson)
    }

    fn clean_json_response(&self, content: &str) -> String {
        let mut cleaned = content.trim().to_string();

        // Remove ```json prefix
        if cleaned.starts_with("```json") {
            cleaned = cleaned.strip_prefix("```json").unwrap().to_string();
        }

        // Remove ``` suffix
        if cleaned.ends_with("```") {
            cleaned = cleaned.strip_suffix("```").unwrap().to_string();
        }

        cleaned.trim().to_string()
    }

    fn generate_lesson_generation_prompt(&self, topic: &str) -> String {
        format!(
            r#"Generate a short lesson for the topic: {topic}.
Please provide a complete lesson in JSON format with the following structure:
{{
  "title": "Lesson title",
  "content": "This is the content of the lesson in markdown format and as an string"
}}

Make sure to use the provided vocabulary words in the examples and exercises.
Return ONLY valid JSON, no additional text."#,
        )
    }
}
