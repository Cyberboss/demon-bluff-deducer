use demon_bluff_gameplay_engine::{game_state::GameState, villager::VillagerIndex};
use log::Log;

use super::{DesireType, HypothesisBuilderType};
use crate::{
	Breakpoint,
	engine::{
		Depth, FitnessAndAction, Hypothesis, HypothesisBuilder, HypothesisEvaluation,
		HypothesisEvaluator, HypothesisFunctions, HypothesisReference, HypothesisRegistrar,
		HypothesisRepository,
	},
	hypotheses::{HypothesisType, is_evil::IsEvilHypothesisBuilder},
	player_action::PlayerAction,
};

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct ExecuteIndexHypothesisBuilder {
	index: VillagerIndex,
}

#[derive(Debug)]
pub struct ExecuteIndexHypothesis {
	index: VillagerIndex,
	is_evil_hypothesis: HypothesisReference,
}

impl ExecuteIndexHypothesisBuilder {
	pub fn new(index: VillagerIndex) -> Self {
		Self { index }
	}
}

impl HypothesisBuilder for ExecuteIndexHypothesisBuilder {
	fn build(
		self,
		game_state: &GameState,
		registrar: &mut impl HypothesisRegistrar<HypothesisBuilderType, DesireType>,
	) -> HypothesisType {
		let is_evil_hypothesis =
			registrar.register(IsEvilHypothesisBuilder::new(self.index.clone()));
		ExecuteIndexHypothesis {
			is_evil_hypothesis,
			index: self.index,
		}
		.into()
	}
}

impl Hypothesis for ExecuteIndexHypothesis {
	fn describe(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
		write!(f, "Execute Villager {}", self.index)
	}

	fn evaluate<TLog, FDebugBreak>(
		&mut self,
		_: &TLog,
		_: Depth,
		game_state: &GameState,
		repository: HypothesisRepository<TLog, FDebugBreak>,
	) -> HypothesisEvaluation
	where
		TLog: Log,
		FDebugBreak: FnMut(Breakpoint) + Clone,
	{
		let estimated_evils =
			(game_state.draw_stats().demons() + game_state.draw_stats().minions()) as f64;
		let total_villagers = game_state.draw_stats().total_villagers() as f64;

		let mut evaluator = repository.require_sub_evaluation(estimated_evils / total_villagers);
		let result = evaluator
			.sub_evaluate(&self.is_evil_hypothesis)
			.map(|fitness| {
				FitnessAndAction::new(
					fitness.fitness(),
					Some(PlayerAction::TryExecute(self.index.clone())),
				)
			});
		evaluator.finalize(result)
	}
}
