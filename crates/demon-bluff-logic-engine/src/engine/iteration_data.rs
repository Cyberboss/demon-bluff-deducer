use serde::Serialize;

use super::HypothesisResult;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub(super) struct IterationData {
    results: Vec<Option<HypothesisResult>>,
}
