//! Provider adapters - map normalized prompts to each provider format

mod anthropic;
mod gemini;
mod openai;

pub use anthropic::AnthropicAdapter;
pub use gemini::GeminiAdapter;
pub use openai::OpenAIAdapter;

use crate::schema::{AIResponse, IdeaBundle};
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait ProviderAdapter: Send + Sync {
    fn provider_name(&self) -> &'static str;
    fn model_name(&self) -> &str;
    async fn complete(&self, system_prompt: &str, user_prompt: &str) -> Result<IdeaBundle, AdapterError>;
}

#[derive(Debug, thiserror::Error)]
pub enum AdapterError {
    #[error("API error: {0}")]
    Api(String),
    #[error("Parse error: {0}")]
    Parse(String),
    #[error("Missing API key")]
    MissingApiKey,
}

fn to_idea_bundle(
    provider: &str,
    model: &str,
    response: AIResponse,
    raw: Option<String>,
) -> IdeaBundle {
    let id = Uuid::new_v4().to_string();
    let created_at = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
        .to_string();
    IdeaBundle {
        id,
        provider: provider.to_string(),
        model: model.to_string(),
        ideas: response.ideas,
        step_plan: response.step_plan,
        risks: response.risks,
        dependencies: response.dependencies,
        effort: response.effort,
        next_actions: response.next_actions,
        raw_response: raw,
        created_at,
    }
}
