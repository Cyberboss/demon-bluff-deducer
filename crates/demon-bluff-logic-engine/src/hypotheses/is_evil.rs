use demon_bluff_gameplay_engine::{
	game_state::GameState,
	villager::{Villager, VillagerArchetype, VillagerIndex},
};
use log::Log;

use super::{
	DesireType, HypothesisBuilderType,
	all_evils_accounted_for::AllEvilsAccountedForHypothesisBuilder,
	is_truly_archetype::IsTrulyArchetypeHypothesisBuilder,
	testimony_condemns::TestimonyCondemnsHypothesisBuilder,
};
use crate::{
	Breakpoint,
	engine::{
		Depth, Hypothesis, HypothesisBuilder, HypothesisEvaluation, HypothesisEvaluator,
		HypothesisFunctions, HypothesisReference, HypothesisRegistrar, HypothesisRepository,
		and_result, or_result,
	},
	hypotheses::{
		HypothesisType, is_corrupt::IsCorruptHypothesisBuilder,
		is_truthful::IsTruthfulHypothesisBuilder, negate::NegateHypothesisBuilder,
	},
};

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct IsEvilHypothesisBuilder {
	index: VillagerIndex,
}

#[derive(Debug)]
pub struct IsEvilHypothesis {
	index: VillagerIndex,
	is_non_liar_hypotheses: Vec<HypothesisReference>,
	testimonies_condemming: Vec<HypothesisReference>,
	testimonies_exonerating: Vec<HypothesisReference>,
	is_lying_hypothesis: HypothesisReference,
	not_corrupt_hypothesis: HypothesisReference,
	any_evil_slots_left_hypothesis: HypothesisReference,
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

		let mut testimonies_condemming = Vec::new();
		game_state.iter_villagers(|index, villager| {
			let potentially_condemning_testimony = match villager {
				Villager::Active(active_villager) => active_villager.instance().testimony(),
				Villager::Hidden(_) => &None,
				Villager::Confirmed(confirmed_villager) => {
					if !confirmed_villager.lies() {
						confirmed_villager.instance().testimony()
					} else {
						&None
					}
				}
			};

			if let Some(testimony) = potentially_condemning_testimony {
				testimonies_condemming.push(registrar.register(
					TestimonyCondemnsHypothesisBuilder::new(
						index,
						testimony.clone(),
						self.index.clone(),
					),
				));
			}
		});

		let testimonies_exonerating = Vec::new(); // TODO

		let is_lying_hypothesis = registrar.register(NegateHypothesisBuilder::new(
			IsTruthfulHypothesisBuilder::new(self.index.clone()),
		));
		let not_corrupt_hypothesis = registrar.register(NegateHypothesisBuilder::new(
			IsCorruptHypothesisBuilder::new(self.index.clone()),
		));

		let any_evil_slots_left_hypothesis = registrar.register(NegateHypothesisBuilder::new(
			AllEvilsAccountedForHypothesisBuilder::default(),
		));

		IsEvilHypothesis {
			index: self.index,
			is_non_liar_hypotheses,
			not_corrupt_hypothesis,
			is_lying_hypothesis,
			testimonies_condemming,
			testimonies_exonerating,
			any_evil_slots_left_hypothesis,
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

		let mut is_a_truthful_evil_result = None;
		for sub_hypothesis in &self.is_non_liar_hypotheses {
			let is_archetype_result = evaluator.sub_evaluate(sub_hypothesis);

			is_a_truthful_evil_result = Some(match is_a_truthful_evil_result {
				Some(other_result) => or_result(other_result, is_archetype_result),
				None => is_archetype_result,
			})
		}

		let evil_2_result = match is_a_truthful_evil_result {
			Some(is_a_truthful_evil_result) => {
				or_result(regular_evil_result, is_a_truthful_evil_result)
			}
			None => regular_evil_result,
		};

		// balance the scales
		let mut testimonies_condemning_result = None;
		for sub_hypothesis in &self.testimonies_condemming {
			let testimony_condemns = evaluator.sub_evaluate(sub_hypothesis);

			testimonies_condemning_result = Some(match testimonies_condemning_result {
				Some(other_result) => or_result(other_result, testimony_condemns),
				None => testimony_condemns,
			});
		}

		let mut testimonies_exonerating_result = None;
		for sub_hypothesis in &self.testimonies_exonerating {
			let testimony_exonerates = evaluator.sub_evaluate(sub_hypothesis);

			testimonies_exonerating_result = Some(match testimonies_exonerating_result {
				Some(other_result) => or_result(other_result, testimony_exonerates),
				None => testimony_exonerates,
			});
		}

		let penultimate_result = match testimonies_condemning_result {
			Some(condeming_result) => match testimonies_exonerating_result {
				Some(exonerating_result) => or_result(
					evil_2_result,
					and_result(
						condeming_result,
						exonerating_result.map(|fitness| fitness.invert()),
					),
				),
				None => or_result(evil_2_result, condeming_result),
			},
			None => match testimonies_exonerating_result {
				Some(exonerating_result) => and_result(
					evil_2_result,
					exonerating_result.map(|fitness| fitness.invert()),
				),
				None => evil_2_result,
			},
		};

		// need this check because any_evil_slots_left_hypothesis is actually dependent on our results which can screw up perfect probabilities
		if penultimate_result.fitness_and_action().is_certain() {
			return evaluator.finalize(penultimate_result);
		}

		let any_evil_slots_left_result =
			evaluator.sub_evaluate(&self.any_evil_slots_left_hypothesis);

		let result = and_result(any_evil_slots_left_result, penultimate_result);

		evaluator.finalize(result)
	}
}
