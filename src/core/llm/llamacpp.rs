use crate::core::llm::Llm;
use crate::core::llm::types::{ChatMessage, ChatRequest, ChatResponse};
use async_trait::async_trait;
use reqwest::Client;

#[derive(Clone)]
pub struct LlamaCppClient {
    base_url: String,
    model: String,
    client: Client,
}

impl LlamaCppClient {
    pub fn new(base_url: String, model: String) -> Self {
        Self {
            base_url,
            model,
            client: Client::new(),
        }
    }
}

#[async_trait]
impl Llm for LlamaCppClient {
    async fn get_response(&self, prompt: &str) -> Result<ChatResponse, Box<dyn std::error::Error>> {
        // Prepare request
        let req_body = ChatRequest {
            model: self.model.clone(),
            stream: false,
            messages: vec![ChatMessage {
                role: "user".to_string(),
                content: prompt.to_string(),
            }],
        };

        // Send POST request
        let url = format!("{}/v1/chat/completions", self.base_url);
        let response = self.client.post(&url).json(&req_body).send().await?;

        // Check if request was successful
        if !response.status().is_success() {
            return Err(format!("Request failed with status: {}", response.status()).into());
        }

        // Parse JSON response
        let chat_response: ChatResponse = response.json().await?;

        Ok(chat_response)
    }
}
