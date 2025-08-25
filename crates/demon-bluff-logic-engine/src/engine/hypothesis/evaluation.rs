use super::result::HypothesisResult;

/// The return value of evaluating a single `Hypothesis`.
#[derive(Debug)]
pub struct HypothesisEvaluation {
    result: HypothesisResult,
}
