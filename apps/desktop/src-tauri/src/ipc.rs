//! Typed IPC layer - Tauri commands for frontend

use idea_engine_core::adapters::{AnthropicAdapter, GeminiAdapter, OpenAIAdapter};
use idea_engine_core::{Orchestrator, Storage};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tauri::State;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateChatInput {
    pub title: String,
    pub template_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SendMessageInput {
    pub chat_id: String,
    pub content: String,
    pub system_prompt: String,
    pub providers: Vec<String>,
    pub rubric: Option<[f64; 6]>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetFeedbackInput {
    pub message_id: String,
    pub feedback: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecipeInput {
    pub id: String,
    pub name: String,
    pub system_prompt: String,
    pub user_prompt_template: String,
    pub rubric_json: String,
    pub few_shot_examples_json: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetApiKeyInput {
    pub provider: String,
    pub key: String,
}

fn get_storage<'a>(state: &'a State<Arc<Storage>>) -> &'a Storage {
    state.inner().as_ref()
}

fn get_api_key(provider: &str) -> Option<String> {
    #[cfg(not(target_os = "macos"))]
    let service = format!("idea-engine-{}", provider);
    #[cfg(target_os = "macos")]
    let service = format!("idea-engine-{}", provider);

    keyring::Entry::new(&service, "api-key")
        .ok()
        .and_then(|e| e.get_password().ok())
}

#[tauri::command]
pub fn create_chat(state: State<Arc<Storage>>, input: CreateChatInput) -> Result<String, String> {
    let id = Uuid::new_v4().to_string();
    get_storage(&state)
        .create_chat(&id, &input.title, input.template_id.as_deref())
        .map_err(|e| e.to_string())?;
    Ok(id)
}

#[tauri::command]
pub fn list_chats(state: State<Arc<Storage>>) -> Result<Vec<serde_json::Value>, String> {
    let rows = get_storage(&state).list_chats().map_err(|e| e.to_string())?;
    Ok(rows
        .into_iter()
        .map(|r| {
            serde_json::json!({
                "id": r.id,
                "title": r.title,
                "templateId": r.template_id,
                "createdAt": r.created_at,
                "updatedAt": r.updated_at,
            })
        })
        .collect())
}

#[tauri::command]
pub fn get_chat_messages(state: State<Arc<Storage>>, chat_id: String) -> Result<Vec<serde_json::Value>, String> {
    let rows = get_storage(&state)
        .get_chat_messages(&chat_id)
        .map_err(|e| e.to_string())?;
    Ok(rows
        .into_iter()
        .map(|r| {
            serde_json::json!({
                "id": r.id,
                "chatId": r.chat_id,
                "role": r.role,
                "content": r.content,
                "ideaBundles": r.idea_bundles_json.and_then(|s| serde_json::from_str::<serde_json::Value>(&s).ok()),
                "feedback": r.feedback,
                "createdAt": r.created_at,
            })
        })
        .collect())
}

#[tauri::command]
pub async fn send_message(state: State<'_, Arc<Storage>>, input: SendMessageInput) -> Result<serde_json::Value, String> {
    let msg_id = Uuid::new_v4().to_string();

    get_storage(&state)
        .insert_message(&msg_id, &input.chat_id, "user", &input.content, None)
        .map_err(|e| e.to_string())?;

    let mut adapters: Vec<Arc<dyn idea_engine_core::adapters::ProviderAdapter>> = Vec::new();
    for p in &input.providers {
        match p.as_str() {
            "openai" => {
                adapters.push(Arc::new(OpenAIAdapter::new(get_api_key("openai"))));
            }
            "anthropic" => {
                adapters.push(Arc::new(AnthropicAdapter::new(get_api_key("anthropic"))));
            }
            "gemini" => {
                adapters.push(Arc::new(GeminiAdapter::new(get_api_key("gemini"))));
            }
            _ => {}
        }
    }

    if adapters.is_empty() {
        return Err("No providers enabled. Add API keys in Settings.".to_string());
    }

    let orchestrator = Orchestrator::new(adapters);
    let result = orchestrator
        .run(&input.system_prompt, &input.content, input.rubric.as_ref())
        .await;

    let bundles_json: Vec<serde_json::Value> = result
        .bundles
        .iter()
        .map(|b| {
            serde_json::json!({
                "id": b.id,
                "provider": b.provider,
                "model": b.model,
                "ideas": b.ideas,
                "stepPlan": b.step_plan,
                "risks": b.risks,
                "dependencies": b.dependencies,
                "effort": b.effort,
                "nextActions": b.next_actions,
                "createdAt": b.created_at,
            })
        })
        .collect();

    let assistant_content = if result.bundles.is_empty() {
        format!(
            "No ideas generated. Errors: {}",
            result
                .errors
                .iter()
                .map(|(p, e)| format!("{}: {}", p, e))
                .collect::<Vec<_>>()
                .join("; ")
        )
    } else {
        format!(
            "Generated {} idea bundle(s). {}",
            result.bundles.len(),
            result
                .errors
                .iter()
                .map(|(p, e)| format!("{}: {}", p, e))
                .collect::<Vec<_>>()
                .join("; ")
        )
        .trim_end_matches("; ")
        .to_string()
    };

    let bundles_str = serde_json::to_string(&bundles_json).unwrap();
    let asst_id = Uuid::new_v4().to_string();
    get_storage(&state)
        .insert_message(&asst_id, &input.chat_id, "assistant", &assistant_content, Some(&bundles_str))
        .map_err(|e| e.to_string())?;

    Ok(serde_json::json!({
        "messageId": asst_id,
        "content": assistant_content,
        "ideaBundles": bundles_json,
        "errors": result.errors,
    }))
}

#[tauri::command]
pub fn set_feedback(state: State<Arc<Storage>>, input: SetFeedbackInput) -> Result<(), String> {
    get_storage(&state)
        .set_message_feedback(&input.message_id, &input.feedback)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn list_recipes(state: State<Arc<Storage>>) -> Result<Vec<serde_json::Value>, String> {
    let rows = get_storage(&state).list_recipes().map_err(|e| e.to_string())?;
    Ok(rows
        .into_iter()
        .map(|r| {
            serde_json::json!({
                "id": r.id,
                "name": r.name,
                "systemPrompt": r.system_prompt,
                "userPromptTemplate": r.user_prompt_template,
                "rubricJson": r.rubric_json,
                "fewShotExamplesJson": r.few_shot_examples_json,
                "createdAt": r.created_at,
            })
        })
        .collect())
}

#[tauri::command]
pub fn save_recipe(state: State<Arc<Storage>>, input: RecipeInput) -> Result<(), String> {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
        .to_string();
    let recipe = idea_engine_core::storage::RecipeRow {
        id: input.id,
        name: input.name,
        system_prompt: input.system_prompt,
        user_prompt_template: input.user_prompt_template,
        rubric_json: input.rubric_json,
        few_shot_examples_json: input.few_shot_examples_json,
        created_at: now,
    };
    get_storage(&state)
        .save_recipe(&recipe)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_api_keys() -> Result<serde_json::Value, String> {
    Ok(serde_json::json!({
        "openai": get_api_key("openai").is_some(),
        "anthropic": get_api_key("anthropic").is_some(),
        "gemini": get_api_key("gemini").is_some(),
    }))
}

#[tauri::command]
pub fn set_api_key(input: SetApiKeyInput) -> Result<(), String> {
    let service = format!("idea-engine-{}", input.provider);
    let entry = keyring::Entry::new(&service, "api-key").map_err(|e| e.to_string())?;
    entry.set_password(&input.key).map_err(|e| e.to_string())
}
