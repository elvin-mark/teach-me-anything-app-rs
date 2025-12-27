use super::types::GeneratedRoadmap;
use crate::core::{agents::utils::clean_json_response, llm::Llm};
use core::str;
use std::sync::Arc;

#[derive(Clone)]
pub struct PlannerAgent {
    llm: Arc<dyn Llm>,
}

impl PlannerAgent {
    pub fn new(llm: Arc<dyn Llm>) -> Arc<Self> {
        Arc::new(Self { llm })
    }

    pub async fn generate_roadmap(
        &self,
        goal: &str,
    ) -> Result<GeneratedRoadmap, Box<dyn std::error::Error>> {
        let prompt = self.roadmap_generation_prompt(goal);

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
        let generated_roadmap: GeneratedRoadmap =
            serde_json::from_str(&cleaned).map_err(|e| format!("Failed to parse JSON: {}", e))?;

        Ok(generated_roadmap)
    }

    fn roadmap_generation_prompt(&self, goal: &str) -> String {
        format!(
            r#"Generate a detail roadmap for the goal: {goal}.
Please provide a complete list of small milestones in JSON format with the following structure:
{{
  "goal": "the goal of the roadmap",
  "milestones": [
    "Milestone 1",
    "Milestone 2",
    "Milestone 3"
  ]
}}

Return ONLY valid JSON, no additional text."#,
        )
    }
}
