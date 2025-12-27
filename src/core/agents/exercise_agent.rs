use crate::core::{
    agents::{
        types::{GeneratedGrading, GeneratedQuestions},
        utils::clean_json_response,
    },
    llm::Llm,
};
use core::str;
use std::sync::Arc;

#[derive(Clone)]
pub struct ExerciseAgent {
    llm: Arc<dyn Llm>,
}

impl ExerciseAgent {
    pub fn new(llm: Arc<dyn Llm>) -> Arc<Self> {
        Arc::new(Self { llm })
    }

    pub async fn generate_questions(
        &self,
        content: &str,
    ) -> Result<GeneratedQuestions, Box<dyn std::error::Error>> {
        let prompt = self.questions_generation_prompt(content);

        let resp = self.llm.get_response(&prompt).await?;

        // Extract content from response
        let content = resp
            .choices
            .first()
            .map(|choice| &choice.message.content)
            .ok_or("No response from LLM")?;

        // Clean the response (remove markdown code blocks)
        let cleaned = clean_json_response(content);

        log::debug!("Response from LLM: {}", cleaned);

        // Parse JSON
        let generated_questions: GeneratedQuestions =
            serde_json::from_str(&cleaned).map_err(|e| format!("Failed to parse JSON: {}", e))?;

        Ok(generated_questions)
    }

    fn questions_generation_prompt(&self, content: &str) -> String {
        format!(
            r#"Generate multiple questions (with a hint to help the user) to check the understanding of the following content: {content}.
Please provide a complete list of questions in JSON format with the following structure:
{{
  "questions": [
    {{
      "question": "Question 1",
      "hint": "Hint for Question 1"
    }},
    {{
      "question": "Question 2",
      "hint": "Hint for Question 2"
    }},
    {{
      "question": "Question 3",
      "hint": "Hint for Question 3"
    }}
  ]
}}

Return ONLY valid JSON, no additional text."#,
        )
    }

    pub async fn grade_question(
        &self,
        question: &str,
        answer: &str,
    ) -> Result<GeneratedGrading, Box<dyn std::error::Error>> {
        let prompt = self.questions_grading_generation_prompt(question, answer);

        let resp = self.llm.get_response(&prompt).await?;

        // Extract content from response
        let content = resp
            .choices
            .first()
            .map(|choice| &choice.message.content)
            .ok_or("No response from LLM")?;

        // Clean the response (remove markdown code blocks)
        let cleaned = clean_json_response(content);

        log::debug!("Response from LLM: {}", cleaned);

        // Parse JSON
        let generated_grading: GeneratedGrading =
            serde_json::from_str(&cleaned).map_err(|e| format!("Failed to parse JSON: {}", e))?;

        Ok(generated_grading)
    }

    fn questions_grading_generation_prompt(&self, question: &str, answer: &str) -> String {
        format!(
            r#"Grade the answer to the following question:
Question: {question}
Answer: {answer}
Please provide you response in JSON format with the following structure:
{{
  "score": <An integer between 0 and 5>,
  "feedback": "A feedback higlighting the strengths and weaknesses of the answer"
}}

Return ONLY valid JSON, no additional text."#,
        )
    }
}
