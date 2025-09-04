use std::arch::breakpoint;

use demon_bluff_gameplay_engine::{
	game_state::GameState,
	villager::{Villager, VillagerArchetype, VillagerIndex},
};
use log::Log;

use super::{
	DesireType, HypothesisBuilderType, is_truly_archetype::IsTrulyArchetypeHypothesisBuilder,
	testimony_condemns_expression::TestimonyCondemnsExpressionHypothesisBuilder,
	testimony_exonerates_expression::TestimonyExoneratesExpressionHypothesisBuilder,
};
use crate::{
	Breakpoint,
	engine::{
		Depth, FitnessAndAction, Hypothesis, HypothesisBuilder, HypothesisEvaluation,
		HypothesisEvaluator, HypothesisFunctions, HypothesisReference, HypothesisRegistrar,
		HypothesisRepository, HypothesisResult, and_result, or_result, sum_result,
	},
	hypotheses::{
		HypothesisType, is_corrupt::IsCorruptHypothesisBuilder,
		is_truthful::IsTruthfulHypothesisBuilder, negate::NegateHypothesisBuilder,
	},
};

#[derive(Debug)]
struct TestimonyHypothesisPair {
	assertion: HypothesisReference,
	truthfulness: HypothesisReference,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct IsEvilHypothesisBuilder {
	index: VillagerIndex,
}

#[derive(Debug)]
pub struct IsEvilHypothesis {
	index: VillagerIndex,
	is_non_liar_hypotheses: Vec<HypothesisReference>,
	testimonies_condemning: Vec<TestimonyHypothesisPair>,
	testimonies_not_exonerating: Vec<TestimonyHypothesisPair>,
	is_lying_hypothesis: HypothesisReference,
	not_lying_hypothesis: HypothesisReference,
	not_corrupt_hypothesis: HypothesisReference,
}

impl IsEvilHypothesisBuilder {
	pub fn new(index: VillagerIndex) -> Self {
		Self { index }
	}
}

impl HypothesisBuilder for IsEvilHypothesisBuilder {
	fn build(
		self,
		game_state: &GameState,
		registrar: &mut impl HypothesisRegistrar<HypothesisBuilderType, DesireType>,
	) -> HypothesisType {
		let mut is_non_liar_hypotheses = Vec::new();
		for archetype in VillagerArchetype::iter() {
			if archetype.is_evil() && !archetype.lies() {
				is_non_liar_hypotheses.push(registrar.register(
					IsTrulyArchetypeHypothesisBuilder::new(archetype, self.index.clone()),
				));
			}
		}

		let mut testimonies_condemning = Vec::new();
		let mut testimonies_not_exonerating = Vec::new();

		game_state.iter_villagers(|index, villager| {
			let testimony_to_consider = match villager {
				Villager::Active(active_villager) => active_villager.instance().testimony(),
				Villager::Hidden(_) => &None,
				Villager::Confirmed(confirmed_villager) => {
					if !confirmed_villager.lies() {
						confirmed_villager.instance().testimony()
					} else {
						&None // TODO: Maybe consider inverse of testimony as truthful? Or is that too risky?
					}
				}
			};

			if let Some(testimony) = testimony_to_consider {
				testimonies_condemning.push(TestimonyHypothesisPair {
					assertion: registrar.register(
						TestimonyCondemnsExpressionHypothesisBuilder::new(
							index.clone(),
							self.index.clone(),
							testimony.clone(),
						),
					),
					truthfulness: registrar
						.register(IsTruthfulHypothesisBuilder::new(index.clone())),
				});
				testimonies_not_exonerating.push(TestimonyHypothesisPair {
					assertion: registrar.register(NegateHypothesisBuilder::new(
						TestimonyExoneratesExpressionHypothesisBuilder::new(
							index.clone(),
							self.index.clone(),
							testimony.clone(),
						),
					)),
					truthfulness: registrar
						.register(IsTruthfulHypothesisBuilder::new(index.clone())),
				});
			}
		});
		let is_lying_hypothesis = registrar.register(NegateHypothesisBuilder::new(
			IsTruthfulHypothesisBuilder::new(self.index.clone()),
		));
		let not_lying_hypothesis =
			registrar.register(IsTruthfulHypothesisBuilder::new(self.index.clone()));
		let not_corrupt_hypothesis = registrar.register(NegateHypothesisBuilder::new(
			IsCorruptHypothesisBuilder::new(self.index.clone()),
		));

		IsEvilHypothesis {
			index: self.index,
			is_non_liar_hypotheses,
			not_corrupt_hypothesis,
			is_lying_hypothesis,
			testimonies_condemning,
			not_lying_hypothesis,
			testimonies_not_exonerating,
		}
		.into()
	}
}

impl Hypothesis for IsEvilHypothesis {
	fn describe(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
		write!(f, "{} is evil", self.index)
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
		let initial_evils = game_state.draw_stats().demons() + game_state.draw_stats().minions();
		let mut evaluator = repository.require_sub_evaluation(
			(initial_evils as f64) / (game_state.draw_stats().total_villagers() as f64),
		);

		let not_corrupt_result = evaluator.sub_evaluate(&self.not_corrupt_hypothesis);
		let lying_result = evaluator.sub_evaluate(&self.is_lying_hypothesis);

		let regular_evil_result = and_result(not_corrupt_result, lying_result.clone());

		let mut is_a_truthful_evil_result = (match game_state.villager(&self.index) {
			Villager::Active(active_villager) => Some(active_villager.instance()),
			Villager::Hidden(_) => None,
			Villager::Confirmed(confirmed_villager) => Some(confirmed_villager.instance()),
		})
		.and_then(|instance| {
			if instance.archetype().cannot_lie() {
				Some(HypothesisResult::Conclusive(FitnessAndAction::certainty(
					None,
				)))
			} else {
				None
			}
		});
		for sub_hypothesis in &self.is_non_liar_hypotheses {
			let is_archetype_result = evaluator.sub_evaluate(sub_hypothesis);

			is_a_truthful_evil_result = Some(match is_a_truthful_evil_result {
				Some(other_result) => or_result(other_result, is_archetype_result),
				None => is_archetype_result,
			})
		}

		let base_evil_result = match is_a_truthful_evil_result {
			Some(is_a_truthful_evil_result) => {
				let not_lying_result = evaluator.sub_evaluate(&self.not_lying_hypothesis);
				or_result(
					regular_evil_result,
					and_result(is_a_truthful_evil_result, not_lying_result),
				)
			}
			None => regular_evil_result,
		};

		// kinda iffy, but assume we weight all testimonies condemning equally with the base probability if they condemn or exonerate at all
		// balance the scales
		let mut penultimate_evil_result = base_evil_result;
		let mut total_components = 1;
		for sub_hypothesis in &self.testimonies_condemning {
			let testimony_condemns = evaluator.sub_evaluate(&sub_hypothesis.assertion);
			if testimony_condemns.fitness_and_action().is_impossible() {
				continue;
			}

			let testimony_is_true = evaluator.sub_evaluate(&sub_hypothesis.truthfulness);

			let testimony_condemns_and_is_true = and_result(testimony_condemns, testimony_is_true);

			penultimate_evil_result =
				sum_result(testimony_condemns_and_is_true, penultimate_evil_result);
			total_components += 1;
		}

		penultimate_evil_result = penultimate_evil_result.map(|fitness_and_action| {
			fitness_and_action.map_action(|fitness| fitness / total_components as f64)
		});

		// exonerations are trickier because they all have to be false to go against an evil
		let mut final_evil_result = penultimate_evil_result;
		for sub_hypothesis in &self.testimonies_not_exonerating {
			// chance we are NOT good/exonerated
			let chance_testimony_doesnt_say_were_good =
				evaluator.sub_evaluate(&sub_hypothesis.assertion);
			if chance_testimony_doesnt_say_were_good
				.fitness_and_action()
				.is_certain()
			{
				// it's just saying that the testimony didn't have anything good to say about us
				// therefore ignore it
				continue;
			}

			// the testimony has SOMETHING good to say about us
			let testimony_is_true = evaluator.sub_evaluate(&sub_hypothesis.truthfulness);

			// the probability that the testimony doesn't assert we're good and truthful
			let chance_testimony_true_and_doesnt_say_were_good =
				and_result(chance_testimony_doesnt_say_were_good, testimony_is_true);

			final_evil_result = and_result(
				chance_testimony_true_and_doesnt_say_were_good,
				final_evil_result,
			);
		}

		evaluator.finalize(final_evil_result)
	}
}
