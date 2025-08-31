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
	hypotheses::is_corrupt::IsCorruptHypothesisBuilder,
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

/// If condemns a given defendent without any truthfulness tests
#[derive(Debug)]
pub struct TestimonyCondemnsHypothesis {
	testifier: VillagerIndex,
	defendant: VillagerIndex,
	testimony_true_hypothesis: HypothesisReference,
	defendant_corrupt_hypothesis: Option<HypothesisReference>,
	expression_friendly: String,
	testimony: Testimony,
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

		let defendant_corrupt_hypothesis =
			match condemns(&self.testimony, &self.defendant, &self.testifier) {
				Condemns::Yes | Condemns::No => None,
				Condemns::IfDefendantNotCorrupt => Some(
					registrar.register(IsCorruptHypothesisBuilder::new(self.defendant.clone())),
				),
			};

		TestimonyCondemnsHypothesis {
			testifier: self.testifier,
			defendant: self.defendant,
			testimony_true_hypothesis,
			expression_friendly,
			testimony: self.testimony,
			defendant_corrupt_hypothesis,
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
					.defendant_corrupt_hypothesis
					.as_ref()
					.expect("Defendant corrupt hypothesis should have been registered!"),
			),
		};

		let testimony_true_result = evaluator.sub_evaluate(&self.testimony_true_hypothesis);

		let final_result = and_result(testimony_true_result, testimony_condemns_result);

		evaluator.finalize(final_result)
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
				panic!("Why does a confession not have the same testifier and defendant?");
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
		| Testimony::NotCorrupt(_)
		| Testimony::Cured(_)
		| Testimony::Architect(_)
		| Testimony::Baker(_)
		| Testimony::Invincible(_)
		| Testimony::Knitter(_)
		| Testimony::Affected(_)
		| Testimony::FakeEvil(_)
		| Testimony::SelfDestruct(_)
		| Testimony::Slayed(_)
		| Testimony::Scout(_) => Condemns::No,
	}
}
