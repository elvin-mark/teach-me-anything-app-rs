pub mod llamacpp;
pub mod types;

use std::sync::Arc;

use crate::core::llm::types::{ChatResponse, LlmConfig};
use async_trait::async_trait;

#[async_trait]
pub trait Llm: Send + Sync {
    async fn get_response(&self, prompt: &str) -> Result<ChatResponse, Box<dyn std::error::Error>>;
}

pub fn new_llm(config: LlmConfig) -> Arc<dyn Llm> {
    match config.llm_type.as_str() {
        "gemini" => {
            todo!("Implement Gemini client")
        }
        _ => Arc::new(llamacpp::LlamaCppClient::new(config.base_url, config.model)),
    }
}
