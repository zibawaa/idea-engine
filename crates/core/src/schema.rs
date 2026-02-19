//! Internal IdeaBundle schema - matches @idea-engine/shared types

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdeaBundle {
    pub id: String,
    pub provider: String,
    pub model: String,
    pub ideas: Vec<Idea>,
    pub step_plan: Vec<Step>,
    pub risks: Vec<Risk>,
    pub dependencies: Vec<String>,
    pub effort: EffortEstimate,
    pub next_actions: Vec<NextAction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_response: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Idea {
    pub title: String,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rationale: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Step {
    pub order: i32,
    pub action: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Risk {
    pub description: String,
    pub severity: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mitigation: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EffortEstimate {
    pub time: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub complexity: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NextAction {
    pub action: String,
    pub priority: String,
}

/// Raw AI response schema - what we expect from providers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIResponse {
    pub ideas: Vec<Idea>,
    pub step_plan: Vec<Step>,
    pub risks: Vec<Risk>,
    pub dependencies: Vec<String>,
    pub effort: EffortEstimate,
    pub next_actions: Vec<NextAction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoreCard {
    pub novelty: f64,
    pub feasibility: f64,
    pub cost: f64,
    pub time: f64,
    pub risk: f64,
    pub clarity: f64,
    pub total: f64,
}
