//! Local eval runner - replays saved problems against prompt recipes, reports deltas

use crate::ranker::Ranker;
use crate::schema::{IdeaBundle, ScoreCard};
use crate::storage::Storage;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvalProblem {
    pub id: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvalReport {
    pub recipe_id: String,
    pub problem_id: String,
    pub bundle_id: String,
    pub score_card: ScoreCard,
    pub delta: Option<DeltaScore>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeltaScore {
    pub novelty: f64,
    pub feasibility: f64,
    pub cost: f64,
    pub time: f64,
    pub risk: f64,
    pub clarity: f64,
}

/// Run eval: load problems from storage, run orchestrator with recipe, compare scores
pub fn run_eval(
    _db_path: impl AsRef<Path>,
    _recipe_id: &str,
    _problem_ids: &[String],
) -> Result<Vec<EvalReport>, String> {
    // Placeholder: in full impl, would load problems, run orchestrator, score, store
    Ok(Vec::new())
}
