use super::{to_idea_bundle, AdapterError, ProviderAdapter};
use crate::schema::{AIResponse, IdeaBundle};
use async_trait::async_trait;
use serde_json::Value;

const MODEL: &str = "gpt-4o-mini";

pub struct OpenAIAdapter {
    api_key: Option<String>,
    model: String,
}

impl OpenAIAdapter {
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
impl ProviderAdapter for OpenAIAdapter {
    fn provider_name(&self) -> &'static str {
        "openai"
    }

    fn model_name(&self) -> &str {
        &self.model
    }

    async fn complete(&self, system_prompt: &str, user_prompt: &str) -> Result<IdeaBundle, AdapterError> {
        let api_key = self
            .api_key
            .as_deref()
            .ok_or(AdapterError::MissingApiKey)?;

        let client = reqwest::Client::new();
        let body = serde_json::json!({
            "model": self.model,
            "messages": [
                { "role": "system", "content": system_prompt },
                { "role": "user", "content": user_prompt }
            ],
            "response_format": {
                "type": "json_schema",
                "json_schema": {
                    "name": "idea_response",
                    "strict": true,
                    "schema": openai_schema()
                }
            },
            "temperature": 0.7
        });

        let res = client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", api_key))
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
        let content = json["choices"][0]["message"]["content"]
            .as_str()
            .ok_or_else(|| AdapterError::Parse("Missing content".into()))?;

        let response: AIResponse = serde_json::from_str(content)
            .map_err(|e| AdapterError::Parse(format!("{}: {}", e, content)))?;

        Ok(to_idea_bundle(
            self.provider_name(),
            &self.model,
            response,
            Some(content.to_string()),
        ))
    }
}

fn openai_schema() -> Value {
    serde_json::json!({
        "type": "object",
        "properties": {
            "ideas": {
                "type": "array",
                "items": {
                    "type": "object",
                    "properties": {
                        "title": { "type": "string" },
                        "description": { "type": "string" },
                        "rationale": { "type": "string" }
                    },
                    "required": ["title", "description"],
                    "additionalProperties": false
                }
            },
            "step_plan": {
                "type": "array",
                "items": {
                    "type": "object",
                    "properties": {
                        "order": { "type": "integer" },
                        "action": { "type": "string" },
                        "details": { "type": "string" }
                    },
                    "required": ["order", "action"],
                    "additionalProperties": false
                }
            },
            "risks": {
                "type": "array",
                "items": {
                    "type": "object",
                    "properties": {
                        "description": { "type": "string" },
                        "severity": { "type": "string", "enum": ["low", "medium", "high"] },
                        "mitigation": { "type": "string" }
                    },
                    "required": ["description", "severity"],
                    "additionalProperties": false
                }
            },
            "dependencies": { "type": "array", "items": { "type": "string" } },
            "effort": {
                "type": "object",
                "properties": {
                    "time": { "type": "string" },
                    "cost": { "type": "string" },
                    "complexity": { "type": "string", "enum": ["low", "medium", "high"] }
                },
                "required": ["time"],
                "additionalProperties": false
            },
            "next_actions": {
                "type": "array",
                "items": {
                    "type": "object",
                    "properties": {
                        "action": { "type": "string" },
                        "priority": { "type": "string", "enum": ["immediate", "short", "medium", "long"] }
                    },
                    "required": ["action", "priority"],
                    "additionalProperties": false
                }
            }
        },
        "required": ["ideas", "step_plan", "risks", "dependencies", "effort", "next_actions"],
        "additionalProperties": false
    })
}
