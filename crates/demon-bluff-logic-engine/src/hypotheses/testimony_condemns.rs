use demon_bluff_gameplay_engine::{
	game_state::GameState,
	testimony::{ConfessorClaim, Testimony},
	villager::VillagerIndex,
};
use log::Log;

use super::{DesireType, HypothesisBuilderType, HypothesisType};
use crate::{
	Breakpoint,
	engine::{
		Depth, FITNESS_UNKNOWN, FitnessAndAction, Hypothesis, HypothesisBuilder,
		HypothesisEvaluation, HypothesisEvaluator, HypothesisFunctions, HypothesisReference,
		HypothesisRegistrar, HypothesisRepository, HypothesisResult,
	},
	hypotheses::{is_corrupt::IsCorruptHypothesisBuilder, negate::NegateHypothesisBuilder},
};

enum Condemns {
	Yes,
	No,
	IfDefendantNotCorrupt,
}

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

/// If a given [`Testimony`] [`Expression`] condemns a give defendant ASSUMING it is true
#[derive(Debug)]
pub struct TestimonyCondemnsHypothesis {
	testifier: VillagerIndex,
	defendant: VillagerIndex,
	defendant_not_corrupt_hypothesis: Option<HypothesisReference>,
	testimony: Testimony,
}

impl HypothesisBuilder for TestimonyCondemnsHypothesisBuilder {
	fn build(
		self,
		_: &GameState,
		registrar: &mut impl HypothesisRegistrar<HypothesisBuilderType, DesireType>,
	) -> HypothesisType {
		let defendant_not_corrupt_hypothesis =
			match condemns(&self.testimony, &self.defendant, &self.testifier) {
				Condemns::Yes | Condemns::No => None,
				Condemns::IfDefendantNotCorrupt => {
					Some(registrar.register(NegateHypothesisBuilder::new(
						IsCorruptHypothesisBuilder::new(self.defendant.clone()),
					)))
				}
			};

		TestimonyCondemnsHypothesis {
			testifier: self.testifier,
			defendant: self.defendant,
			testimony: self.testimony,
			defendant_not_corrupt_hypothesis,
		}
		.into()
	}
}

impl Hypothesis for TestimonyCondemnsHypothesis {
	fn describe(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
		write!(
			f,
			"{}'s unary testimony condemns {}: {}",
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
		let testimony_condemns = condemns(&self.testimony, &self.defendant, &self.testifier);

		let mut evaluator = repository.require_sub_evaluation(FITNESS_UNKNOWN);

		let testimony_condemns_result = match testimony_condemns {
			Condemns::Yes => HypothesisResult::Conclusive(FitnessAndAction::certainty(None)),
			Condemns::No => HypothesisResult::Conclusive(FitnessAndAction::impossible()),
			Condemns::IfDefendantNotCorrupt => evaluator.sub_evaluate(
				&self
					.defendant_not_corrupt_hypothesis
					.as_ref()
					.expect("Defendant corrupt hypothesis should have been registered!"),
			),
		};

		evaluator.finalize(testimony_condemns_result)
	}
}

fn condemns(
	testimony: &Testimony,
	defendant: &VillagerIndex,
	testifier: &VillagerIndex,
) -> Condemns {
	match testimony {
		Testimony::Confess(confession) => {
			if defendant == testifier {
				match confession {
					ConfessorClaim::Good => Condemns::No,
					ConfessorClaim::Dizzy => Condemns::IfDefendantNotCorrupt,
				}
			} else {
				Condemns::No
			}
		}
		Testimony::Evil(villager_index) => {
			if defendant == villager_index {
				Condemns::Yes
			} else {
				Condemns::No
			}
		}
		Testimony::Lying(villager_index) => {
			if defendant == villager_index {
				Condemns::IfDefendantNotCorrupt
			} else {
				Condemns::No
			}
		}
		Testimony::Role(role_claim) => {
			if role_claim.role().is_evil() && role_claim.index() == defendant {
				Condemns::Yes
			} else {
				Condemns::No
			}
		}
		Testimony::Good(_)
		| Testimony::Corrupt(_)
		| Testimony::Cured(_)
		| Testimony::Baker(_)
		| Testimony::Invincible(_)
		| Testimony::Affected(_)
		| Testimony::FakeEvil(_)
		| Testimony::SelfDestruct(_)
		| Testimony::Slayed(_)
		| Testimony::Scout(_) => Condemns::No,
	}
}
