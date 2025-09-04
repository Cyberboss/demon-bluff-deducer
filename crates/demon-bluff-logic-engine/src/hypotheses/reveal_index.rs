use demon_bluff_gameplay_engine::{game_state::GameState, villager::VillagerIndex};
use log::Log;

use super::{DesireType, HypothesisBuilderType};
use crate::{
	Breakpoint,
	engine::{
		Depth, FitnessAndAction, Hypothesis, HypothesisBuilder, HypothesisEvaluation,
		HypothesisEvaluator, HypothesisFunctions, HypothesisReference, HypothesisRegistrar,
		HypothesisRepository, and_result,
	},
	hypotheses::{
		HypothesisType, need_testimony::NeedTestimonyHypothesisBuilder,
		revealing_is_safe::RevealingIsSafeHypothesisBuilder,
	},
	player_action::PlayerAction,
};

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct RevealIndexHypothesisBuilder {
	index: VillagerIndex,
}

#[derive(Debug)]
pub struct RevealIndexHypothesis {
	index: VillagerIndex,
	revealing_is_safe_hypothesis: HypothesisReference,
	need_testimony_hypothesis: HypothesisReference,
}

impl RevealIndexHypothesisBuilder {
	pub fn new(index: VillagerIndex) -> Self {
		Self { index }
	}
}

impl HypothesisBuilder for RevealIndexHypothesisBuilder {
	fn build(
		self,
		_: &GameState,
		registrar: &mut impl HypothesisRegistrar<HypothesisBuilderType, DesireType>,
	) -> HypothesisType {
		let revealing_is_safe_hypothesis =
			registrar.register(RevealingIsSafeHypothesisBuilder::default());
		let need_testimony_hypothesis =
			registrar.register(NeedTestimonyHypothesisBuilder::new(self.index.clone()));
		RevealIndexHypothesis {
			index: self.index,
			revealing_is_safe_hypothesis,
			need_testimony_hypothesis,
		}
		.into()
	}
}

impl Hypothesis for RevealIndexHypothesis {
	fn describe(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
		write!(f, "Reveal {}", self.index)
	}

	fn evaluate<TLog, FDebugBreak>(
		&mut self,
		_: &TLog,
		_: Depth,
		_: &GameState,
		repository: HypothesisRepository<TLog, FDebugBreak>,
	) -> HypothesisEvaluation
	where
		TLog: Log,
		FDebugBreak: FnMut(Breakpoint) + Clone,
	{
		let mut evaluator = repository.require_sub_evaluation(0.0);

		let revealing_safe_result = evaluator.sub_evaluate(&self.revealing_is_safe_hypothesis);

		let info_desire_result = evaluator.sub_evaluate(&self.need_testimony_hypothesis);

		let fitness = and_result(revealing_safe_result, info_desire_result);

		let result = fitness.map(|fitness| {
			FitnessAndAction::new(
				fitness.fitness(),
				Some(PlayerAction::TryReveal(self.index.clone())),
			)
		});

		evaluator.finalize(result)
	}
}
