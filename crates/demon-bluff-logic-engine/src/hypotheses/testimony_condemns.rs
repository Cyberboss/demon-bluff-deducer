use demon_bluff_gameplay_engine::{
	Expression,
	game_state::GameState,
	testimony::{ConfessorClaim, Testimony},
	villager::{Villager, VillagerIndex},
};
use log::Log;

use super::{
	DesireType, HypothesisBuilderType, HypothesisType, testimony::TestimonyHypothesisBuilder,
};
use crate::{
	Breakpoint,
	engine::{
		Depth, FITNESS_UNKNOWN, FitnessAndAction, Hypothesis, HypothesisBuilder,
		HypothesisEvaluation, HypothesisEvaluator, HypothesisFunctions, HypothesisReference,
		HypothesisRegistrar, HypothesisRepository, HypothesisResult, and_result,
	},
};

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct TestimonyCondemnsHypothesisBuilder {
	testifier: VillagerIndex,
	defendant: VillagerIndex,
	testimony: Testimony,
}

impl TestimonyCondemnsHypothesisBuilder {
	pub fn new(testifier: VillagerIndex, defendant: VillagerIndex, testimony: Testimony) -> Self {
		Self {
			testifier,
			defendant,
			testimony,
		}
	}
}

/// If condemns a given defendent without any truthfulness tests
#[derive(Debug)]
pub struct TestimonyCondemnsHypothesis {
	testifier: VillagerIndex,
	defendant: VillagerIndex,
	testimony_true_hypothesis: HypothesisReference,
	expression_friendly: String,
}

impl HypothesisBuilder for TestimonyCondemnsHypothesisBuilder {
	fn build(
		self,
		_: &GameState,
		registrar: &mut impl HypothesisRegistrar<HypothesisBuilderType, DesireType>,
	) -> HypothesisType {
		let expression_friendly = format!("{}", self.testimony);
		let testimony_true_hypothesis = registrar.register(TestimonyHypothesisBuilder::new(
			self.testifier.clone(),
			self.testimony.clone(),
		));
		TestimonyCondemnsHypothesis {
			testifier: self.testifier,
			defendant: self.defendant,
			testimony_true_hypothesis,
			expression_friendly,
		}
		.into()
	}
}

impl Hypothesis for TestimonyCondemnsHypothesis {
	fn describe(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
		write!(
			f,
			"{}'s unary testimony condemns {}: {}",
			self.testifier, self.defendant, self.expression_friendly,
		)
	}

	fn wip(&self) -> bool {
		true
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
		let testimony_condemns_result = self.testimony_condemns(game_state);

		let mut evaluator = repository.require_sub_evaluation(FITNESS_UNKNOWN);

		let testimony_true_result = evaluator.sub_evaluate(&self.testimony_true_hypothesis);

		let final_result = and_result(testimony_true_result, testimony_condemns_result);

		evaluator.finalize(final_result)
	}
}

impl TestimonyCondemnsHypothesis {
	fn testimony_condemns(&self, game_state: &GameState) -> HypothesisResult {
		let testimony = match game_state.villager(&self.testifier) {
			Villager::Active(active_villager) => active_villager.instance().testimony(),
			Villager::Hidden(_) => {
				return HypothesisResult::Conclusive(FitnessAndAction::impossible());
			}
			Villager::Confirmed(confirmed_villager) => confirmed_villager.instance().testimony(),
		};

		let expression = match testimony {
			Some(testimony) => testimony,
			None => {
				return HypothesisResult::Conclusive(FitnessAndAction::impossible());
			}
		};

		match expression {
			Expression::Unary(Testimony::Confess(confession)) => {
				if self.defendant == self.testifier {
					HypothesisResult::Conclusive(match confession {
						ConfessorClaim::Good => FitnessAndAction::impossible(),
						ConfessorClaim::Dizzy => FitnessAndAction::certainty(None),
					})
				} else {
					HypothesisResult::Conclusive(FitnessAndAction::impossible())
				}
			}
			_ => HypothesisResult::Conclusive(FitnessAndAction::unimplemented()),
		}
	}
}
