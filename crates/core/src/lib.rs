//! Idea Engine Core - Domain logic for AI orchestration, adapters, ranker

pub mod adapters;
pub mod eval;
pub mod orchestrator;
pub mod ranker;
pub mod schema;
pub mod storage;

pub use adapters::{AnthropicAdapter, GeminiAdapter, OpenAIAdapter};
pub use orchestrator::Orchestrator;
pub use ranker::Ranker;
pub use schema::IdeaBundle;
pub use storage::{RecipeRow, Storage};
