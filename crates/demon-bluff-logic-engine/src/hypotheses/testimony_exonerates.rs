use demon_bluff_gameplay_engine::{
	game_state::GameState,
	testimony::{ConfessorClaim, Testimony},
	villager::VillagerIndex,
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

enum Exonerates {
	Yes,
	No,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct TestimonyExoneratesHypothesisBuilder {
	testifier: VillagerIndex,
	defendant: VillagerIndex,
	testimony: Testimony,
}

impl TestimonyExoneratesHypothesisBuilder {
	pub fn new(testifier: VillagerIndex, defendant: VillagerIndex, testimony: Testimony) -> Self {
		Self {
			testifier,
			defendant,
			testimony,
		}
	}
}

/// If a testimony is true and exonerates a given defendent
#[derive(Debug)]
pub struct TestimonyExoneratesHypothesis {
	testifier: VillagerIndex,
	defendant: VillagerIndex,
	testimony_true_hypothesis: HypothesisReference,
	testimony: Testimony,
}

impl HypothesisBuilder for TestimonyExoneratesHypothesisBuilder {
	fn build(
		self,
		_: &GameState,
		registrar: &mut impl HypothesisRegistrar<HypothesisBuilderType, DesireType>,
	) -> HypothesisType {
		let testimony_true_hypothesis = registrar.register(TestimonyHypothesisBuilder::new(
			self.testifier.clone(),
			self.testimony.clone(),
		));

		TestimonyExoneratesHypothesis {
			testifier: self.testifier,
			defendant: self.defendant,
			testimony_true_hypothesis,
			testimony: self.testimony,
		}
		.into()
	}
}

impl Hypothesis for TestimonyExoneratesHypothesis {
	fn describe(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
		write!(
			f,
			"{}'s unary testimony exonerates {}: {}",
			self.testifier, self.defendant, self.testimony,
		)
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
		let testimony_exonerates = exonerates(&self.testimony, &self.defendant, &self.testifier);

		let mut evaluator = repository.require_sub_evaluation(FITNESS_UNKNOWN);

		let testimony_exonerates_result = HypothesisResult::Conclusive(if testimony_exonerates {
			FitnessAndAction::certainty(None)
		} else {
			FitnessAndAction::impossible()
		});

		let testimony_true_result = evaluator.sub_evaluate(&self.testimony_true_hypothesis);

		let final_result = and_result(testimony_true_result, testimony_exonerates_result);

		evaluator.finalize(final_result)
	}
}

fn exonerates(testimony: &Testimony, defendant: &VillagerIndex, testifier: &VillagerIndex) -> bool {
	match testimony {
		Testimony::Confess(confession) => {
			if defendant == testifier {
				match confession {
					ConfessorClaim::Good => true,
					ConfessorClaim::Dizzy => false,
				}
			} else {
				false
			}
		}
		Testimony::Role(role_claim) => {
			!role_claim.role().is_evil() && role_claim.index() == defendant
		}
		Testimony::Invincible(_) | Testimony::Good(_) | Testimony::Corrupt(_) => true,
		Testimony::Evil(_)
		| Testimony::Lying(_)
		| Testimony::NotCorrupt(_)
		| Testimony::Cured(_)
		| Testimony::Architect(_)
		| Testimony::Baker(_)
		| Testimony::Knitter(_)
		| Testimony::Affected(_)
		| Testimony::FakeEvil(_)
		| Testimony::SelfDestruct(_)
		| Testimony::Slayed(_)
		| Testimony::Scout(_) => false,
	}
}
