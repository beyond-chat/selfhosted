use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PromptConfig {
    pub id: u32,
    pub name: String,
    pub max_tokens: u32,
    pub temperature: f32,
    pub system_prompt: String,
}
#[derive(Serialize, Deserialize)]
pub enum LlmSdk {
    OpenAI,
    Anthropic,
}
#[derive(Serialize, Deserialize)]
pub struct ApiAndModelsConfig {
    pub id: u32,
    pub name: String,
    pub endpoint_sdk: LlmSdk,
    #[serde(skip_serializing_if = "should_skip_api_key")]
    pub api_key: Option<String>,
    pub base_url: String,
    pub models: Vec<String>,
}

// Custom serialization function to skip the API key or return an empty string
fn should_skip_api_key(api_key: &Option<String>) -> bool {
    api_key.is_none() || api_key.as_deref() == Some("")
}
impl ApiAndModelsConfig {
    pub fn without_api_key(mut self) -> Self {
        self.api_key = Some(String::new()); // Set api_key to an empty string
        self
    }
}
