//! Ranker - apply rubric, dedupe, pick top ideas

use crate::schema::{IdeaBundle, ScoreCard};

const DEFAULT_RUBRIC: [f64; 6] = [1.5, 2.0, 1.0, 1.0, 1.5, 2.0]; // novelty, feasibility, cost, time, risk, clarity

pub struct Ranker {
    rubric: [f64; 6],
}

impl Ranker {
    pub fn new(rubric: Option<&[f64; 6]>) -> Self {
        Self {
            rubric: rubric.map(|r| *r).unwrap_or(DEFAULT_RUBRIC),
        }
    }

    /// Score each bundle, dedupe by similarity, return sorted by total score
    pub fn rank_and_merge(&self, bundles: Vec<IdeaBundle>) -> Vec<IdeaBundle> {
        let scored: Vec<(IdeaBundle, ScoreCard)> = bundles
            .into_iter()
            .map(|b| {
                let card = self.score_bundle(&b);
                (b, card)
            })
            .collect();

        let mut deduped = dedupe_similar(scored);
        deduped.sort_by(|a, b| b.1.total.partial_cmp(&a.1.total).unwrap_or(std::cmp::Ordering::Equal));
        deduped.into_iter().map(|(b, _)| b).collect()
    }

    fn score_bundle(&self, bundle: &IdeaBundle) -> ScoreCard {
        let novelty = heuristic_novelty(bundle);
        let feasibility = heuristic_feasibility(bundle);
        let cost = heuristic_cost(bundle);
        let time = heuristic_time(bundle);
        let risk = heuristic_risk(bundle);
        let clarity = heuristic_clarity(bundle);

        let total = novelty * self.rubric[0]
            + feasibility * self.rubric[1]
            + cost * self.rubric[2]
            + time * self.rubric[3]
            + risk * self.rubric[4]
            + clarity * self.rubric[5];

        ScoreCard {
            novelty,
            feasibility,
            cost,
            time,
            risk,
            clarity,
            total,
        }
    }
}

fn heuristic_novelty(bundle: &IdeaBundle) -> f64 {
    let idea_count = bundle.ideas.len() as f64;
    let has_rationale = bundle.ideas.iter().filter(|i| i.rationale.is_some()).count() as f64;
    ((idea_count * 0.5) + (has_rationale * 0.5)).min(10.0)
}

fn heuristic_feasibility(bundle: &IdeaBundle) -> f64 {
    let step_count = bundle.step_plan.len() as f64;
    let has_details = bundle.step_plan.iter().filter(|s| s.details.is_some()).count() as f64;
    ((step_count.min(5.0) * 1.0) + (has_details * 0.5)).min(10.0)
}

fn heuristic_cost(bundle: &IdeaBundle) -> f64 {
    if bundle.effort.cost.is_some() {
        7.0
    } else {
        5.0
    }
}

fn heuristic_time(bundle: &IdeaBundle) -> f64 {
    if !bundle.effort.time.is_empty() {
        7.0
    } else {
        4.0
    }
}

fn heuristic_risk(bundle: &IdeaBundle) -> f64 {
    let high = bundle.risks.iter().filter(|r| r.severity == "high").count();
    let med = bundle.risks.iter().filter(|r| r.severity == "medium").count();
    let with_mitigation = bundle.risks.iter().filter(|r| r.mitigation.is_some()).count();
    let base = 10.0 - (high as f64 * 2.0) - (med as f64 * 0.5);
    (base + (with_mitigation as f64 * 0.3)).max(0.0).min(10.0)
}

fn heuristic_clarity(bundle: &IdeaBundle) -> f64 {
    let idea_words: usize = bundle.ideas.iter().map(|i| i.description.split_whitespace().count()).sum();
    let step_words: usize = bundle.step_plan.iter().map(|s| s.action.split_whitespace().count()).sum();
    let total = idea_words + step_words;
    (total as f64 / 10.0).min(10.0)
}

fn dedupe_similar(bundles: Vec<(IdeaBundle, ScoreCard)>) -> Vec<(IdeaBundle, ScoreCard)> {
    let mut result = Vec::new();
    for (bundle, card) in bundles {
        let is_dup = result.iter().any(|(existing, _): &(IdeaBundle, ScoreCard)| {
            let a_titles: Vec<_> = existing.ideas.iter().map(|i| i.title.as_str()).collect();
            let b_titles: Vec<_> = bundle.ideas.iter().map(|i| i.title.as_str()).collect();
            jaccard_similarity(&a_titles, &b_titles) > 0.7
        });
        if !is_dup {
            result.push((bundle, card));
        }
    }
    result
}

fn jaccard_similarity(a: &[&str], b: &[&str]) -> f64 {
    if a.is_empty() && b.is_empty() {
        return 1.0;
    }
    let set_a: std::collections::HashSet<_> = a.iter().collect();
    let set_b: std::collections::HashSet<_> = b.iter().collect();
    let inter = set_a.intersection(&set_b).count();
    let union = set_a.union(&set_b).count();
    if union == 0 {
        0.0
    } else {
        inter as f64 / union as f64
    }
}
