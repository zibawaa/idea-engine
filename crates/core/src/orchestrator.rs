//! Orchestrator - fan out to providers, timeout, retry

use crate::adapters::{AdapterError, ProviderAdapter};
use crate::ranker::Ranker;
use crate::schema::IdeaBundle;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::timeout;

const DEFAULT_TIMEOUT_SECS: u64 = 60;
const DEFAULT_RETRIES: u32 = 2;

pub struct Orchestrator {
    adapters: Vec<Arc<dyn ProviderAdapter>>,
    timeout_secs: u64,
    retries: u32,
}

impl Orchestrator {
    pub fn new(adapters: Vec<Arc<dyn ProviderAdapter>>) -> Self {
        Self {
            adapters,
            timeout_secs: DEFAULT_TIMEOUT_SECS,
            retries: DEFAULT_RETRIES,
        }
    }

    pub fn with_timeout(mut self, secs: u64) -> Self {
        self.timeout_secs = secs;
        self
    }

    pub fn with_retries(mut self, retries: u32) -> Self {
        self.retries = retries;
        self
    }

    /// Fan out the same prompt to all adapters, collect results, rank and merge
    pub async fn run(
        &self,
        system_prompt: &str,
        user_prompt: &str,
        rubric: Option<&[f64; 6]>,
    ) -> OrchestratorResult {
        let mut tasks = Vec::with_capacity(self.adapters.len());
        for adapter in &self.adapters {
            let sys = system_prompt.to_string();
            let usr = user_prompt.to_string();
            let ad = Arc::clone(adapter);
            let timeout_secs = self.timeout_secs;
            let retries = self.retries;
            tasks.push(tokio::spawn(async move {
                run_with_retry(&*ad, &sys, &usr, timeout_secs, retries).await
            }));
        }

        let mut bundles = Vec::new();
        let mut errors = Vec::new();

        for (i, task) in tasks.into_iter().enumerate() {
            match task.await {
                Ok(Ok(bundle)) => bundles.push(bundle),
                Ok(Err(e)) => errors.push((self.adapters[i].provider_name().to_string(), e.to_string())),
                Err(e) => errors.push((self.adapters[i].provider_name().to_string(), e.to_string())),
            }
        }

        let ranked = if bundles.is_empty() {
            Vec::new()
        } else {
            let ranker = Ranker::new(rubric);
            ranker.rank_and_merge(bundles)
        };

        OrchestratorResult {
            bundles: ranked,
            errors,
        }
    }
}

async fn run_with_retry(
    adapter: &dyn ProviderAdapter,
    system_prompt: &str,
    user_prompt: &str,
    timeout_secs: u64,
    retries: u32,
) -> Result<IdeaBundle, AdapterError> {
    let mut last_err = None;
    for _ in 0..=retries {
        let result = timeout(
            Duration::from_secs(timeout_secs),
            adapter.complete(system_prompt, user_prompt),
        )
        .await;

        match result {
            Ok(Ok(bundle)) => return Ok(bundle),
            Ok(Err(e)) => last_err = Some(e),
            Err(_) => last_err = Some(AdapterError::Api("Timeout".into())),
        }
    }
    Err(last_err.unwrap_or(AdapterError::Api("Unknown".into())))
}

pub struct OrchestratorResult {
    pub bundles: Vec<IdeaBundle>,
    pub errors: Vec<(String, String)>,
}
