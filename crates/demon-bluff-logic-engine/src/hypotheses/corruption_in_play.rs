use demon_bluff_gameplay_engine::{
	affect::Affect, game_state::GameState, villager::VillagerArchetype,
};
use log::Log;

use super::{DesireType, HypothesisBuilderType};
use crate::{
	Breakpoint,
	engine::{
		Depth, FITNESS_UNKNOWN, Hypothesis, HypothesisBuilder, HypothesisEvaluation,
		HypothesisEvaluator, HypothesisFunctions, HypothesisReference, HypothesisRegistrar,
		HypothesisRepository, HypothesisResult, or_result,
	},
	hypotheses::{HypothesisType, archetype_in_play::ArchetypeInPlayHypothesisBuilder},
};

#[derive(Eq, PartialEq, Debug, Clone, Default)]
pub struct CorruptionInPlayHypothesisBuilder {}

/// Evaluates if corruption is in play
#[derive(Debug)]
pub struct CorruptionInPlayHypothesis {
	corrupting_archetype_hypotheses: Vec<HypothesisReference>,
}

impl HypothesisBuilder for CorruptionInPlayHypothesisBuilder {
	fn build(
		self,
		game_state: &GameState,
		registrar: &mut impl HypothesisRegistrar<HypothesisBuilderType, DesireType>,
	) -> HypothesisType {
		let mut corrupting_archetype_hypotheses = Vec::new();
		for archetype in VillagerArchetype::iter() {
			if let Some(Affect::Corrupt(_)) = archetype.affect(game_state.total_villagers(), None) {
				corrupting_archetype_hypotheses
					.push(registrar.register(ArchetypeInPlayHypothesisBuilder::new(archetype)));
			}
		}

		CorruptionInPlayHypothesis {
			corrupting_archetype_hypotheses,
		}
		.into()
	}
}

impl Hypothesis for CorruptionInPlayHypothesis {
	fn describe(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
		write!(f, "Corruption in play")
	}

	fn wip(&self) -> bool {
		true
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
		if self.corrupting_archetype_hypotheses.is_empty() {
			return repository.finalize(HypothesisResult::impossible());
		}

		let mut evaluator = repository.require_sub_evaluation(FITNESS_UNKNOWN);

		let mut sub_result = None;
		for sub_hypothesis in &self.corrupting_archetype_hypotheses {
			let new_sub_result = evaluator.sub_evaluate(sub_hypothesis);
			sub_result = Some(match sub_result {
				Some(old_sub_result) => or_result(old_sub_result, new_sub_result),
				None => new_sub_result,
			})
		}

		evaluator.finalize(sub_result.expect("Logic error in CorruptionInPlayHypothesis"))
	}
}
