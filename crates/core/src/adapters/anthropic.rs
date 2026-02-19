use super::{to_idea_bundle, AdapterError, ProviderAdapter};
use crate::schema::{AIResponse, IdeaBundle};
use async_trait::async_trait;
use serde_json::Value;

const MODEL: &str = "claude-3-5-haiku-20241022";

pub struct AnthropicAdapter {
    api_key: Option<String>,
    model: String,
}

impl AnthropicAdapter {
    pub fn new(api_key: Option<String>) -> Self {
        Self {
            api_key,
            model: MODEL.to_string(),
        }
    }

    pub fn with_model(mut self, model: &str) -> Self {
        self.model = model.to_string();
        self
    }
}

#[async_trait]
impl ProviderAdapter for AnthropicAdapter {
    fn provider_name(&self) -> &'static str {
        "anthropic"
    }

    fn model_name(&self) -> &str {
        &self.model
    }

    async fn complete(&self, system_prompt: &str, user_prompt: &str) -> Result<IdeaBundle, AdapterError> {
        let api_key = self
            .api_key
            .as_deref()
            .ok_or(AdapterError::MissingApiKey)?;

        let system_with_schema = format!(
            "{}\n\nRespond with valid JSON matching this schema: ideas (array of {{title, description, rationale?}}), step_plan (array of {{order, action, details?}}), risks (array of {{description, severity, mitigation?}}), dependencies (array of strings), effort ({{time, cost?, complexity?}}), next_actions (array of {{action, priority}}).",
            system_prompt
        );

        let client = reqwest::Client::new();
        let body = serde_json::json!({
            "model": self.model,
            "max_tokens": 4096,
            "system": system_with_schema,
            "messages": [
                { "role": "user", "content": user_prompt }
            ],
            "temperature": 0.7
        });

        let res = client
            .post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", api_key)
            .header("anthropic-version", "2023-06-01")
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e| AdapterError::Api(e.to_string()))?;

        let status = res.status();
        let text = res.text().await.map_err(|e| AdapterError::Api(e.to_string()))?;

        if !status.is_success() {
            return Err(AdapterError::Api(format!("{}: {}", status, text)));
        }

        let json: Value = serde_json::from_str(&text).map_err(|e| AdapterError::Parse(e.to_string()))?;
        let content = json["content"]
            .as_array()
            .and_then(|arr| arr.first())
            .and_then(|c| c["text"].as_str())
            .ok_or_else(|| AdapterError::Parse("Missing content".into()))?;

        let response = parse_json_response(content)?;
        Ok(to_idea_bundle(
            self.provider_name(),
            &self.model,
            response,
            Some(content.to_string()),
        ))
    }
}

fn parse_json_response(content: &str) -> Result<AIResponse, AdapterError> {
    let trimmed = content.trim();
    let json_str = if trimmed.starts_with("```json") {
        trimmed.trim_start_matches("```json").trim_end_matches("```").trim()
    } else if trimmed.starts_with("```") {
        trimmed.trim_start_matches("```").trim_end_matches("```").trim()
    } else {
        trimmed
    };
    serde_json::from_str(json_str).map_err(|e| AdapterError::Parse(format!("{}: {}", e, json_str)))
}
