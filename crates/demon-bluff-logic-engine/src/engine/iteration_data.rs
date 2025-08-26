use serde::Serialize;

use super::HypothesisResult;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct IterationData {
    pub results: Vec<Option<HypothesisResult>>,
}
