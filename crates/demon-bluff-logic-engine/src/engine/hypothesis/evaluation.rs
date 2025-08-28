use super::result::HypothesisResult;

/// The return value of evaluating a single `Hypothesis`.
#[derive(Debug)]
pub struct HypothesisEvaluation {
	result: HypothesisResult,
}

impl HypothesisEvaluation {
	pub(super) fn new(result: HypothesisResult) -> Self {
		Self { result }
	}

	pub(super) fn unpack(self) -> HypothesisResult {
		self.result
	}
}
