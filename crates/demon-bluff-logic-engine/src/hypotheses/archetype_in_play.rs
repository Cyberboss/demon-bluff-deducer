use std::collections::HashMap;

use demon_bluff_gameplay_engine::{
	affect::Affect,
	game_state::GameState,
	villager::VillagerArchetype,
};
use log::Log;

use super::{DesireType, HypothesisBuilderType};
use crate::{
	Breakpoint,
	engine::{
		Depth, FitnessAndAction, Hypothesis, HypothesisBuilder, HypothesisEvaluation,
		HypothesisFunctions, HypothesisReference, HypothesisRegistrar, HypothesisRepository,
		HypothesisResult,
	},
	hypotheses::HypothesisType,
};

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct ArchetypeInPlayHypothesisBuilder {
	archetype: VillagerArchetype,
}

#[derive(Debug)]
pub struct ArchetypeInPlayHypothesis {
	archetype: VillagerArchetype,
	converter_in_play_hypotheses: HashMap<VillagerArchetype, HypothesisReference>,
}

impl ArchetypeInPlayHypothesisBuilder {
	pub fn new(archetype: VillagerArchetype) -> Self {
		Self { archetype }
	}
}

impl HypothesisBuilder for ArchetypeInPlayHypothesisBuilder {
	fn build(
		self,
		game_state: &GameState,
		registrar: &mut impl HypothesisRegistrar<HypothesisBuilderType, DesireType>,
	) -> HypothesisType {
		let mut converter_in_play_hypotheses = HashMap::new();

		if let VillagerArchetype::GoodVillager(_) = &self.archetype {
			for archetype in VillagerArchetype::iter() {
				if archetype == self.archetype {
					continue;
				}

				if let Some(affect) = archetype.affect(game_state.total_villagers(), None) {
					match affect {
						Affect::Puppet(_) | Affect::Outcast(_) => {
							converter_in_play_hypotheses.insert(
								archetype.clone(),
								registrar
									.register(ArchetypeInPlayHypothesisBuilder::new(archetype)),
							);
						}
						_ => {}
					}
				}
			}
		}

		ArchetypeInPlayHypothesis {
			archetype: self.archetype,
			converter_in_play_hypotheses,
		}
		.into()
	}
}

impl Hypothesis for ArchetypeInPlayHypothesis {
	fn describe(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
		write!(f, "{} in play", self.archetype)
	}

	fn wip(&self) -> bool {
		true
	}

	fn evaluate<TLog, FDebugBreak>(
		&mut self,
		log: &TLog,
		depth: Depth,
		game_state: &GameState,
		repository: HypothesisRepository<TLog, FDebugBreak>,
	) -> HypothesisEvaluation
	where
		TLog: Log,
		FDebugBreak: FnMut(Breakpoint) + Clone,
	{
		// special case: remove proability if card has a night effect and night effects aren't active
		let has_night_effect = if let Some(Affect::Night(_)) =
			self.archetype.affect(game_state.total_villagers(), None)
		{
			true
		} else {
			false
		};

		if has_night_effect && !game_state.night_actions_in_play() {
			return repository.finalize(HypothesisResult::impossible());
		}

		// step one, eliminate the possibility if it's not in the deck
		// also collect specific conversion possibilities
		// currently the only case where an archetype not in the deck can appear is the puppeteer
		let mut can_be_adjacent_converted_to_by = HashMap::new();
		let mut can_have_conversion_stolen_by_adjacent = HashMap::new();
		let mut can_be_converted_away_from_by_adjacent = HashMap::new();
		let mut in_deck = false;
		for archetype in game_state.deck() {
			if *archetype == self.archetype {
				in_deck = true;
				break;
			}

			// index doesn't matter for this question
			if let Some(affect) = archetype.affect(game_state.total_villagers(), None) {
				match affect {
					Affect::Corrupt(_)
					| Affect::Night(_)
					| Affect::DupeVillager
					| Affect::FakeOutcast
					| Affect::BlockLastNReveals(_) => {}
					Affect::Outcast(_) => {
						for archetype in VillagerArchetype::iter().filter(|archetype| {
							if let Some(Affect::Outcast(_)) =
								archetype.affect(game_state.total_villagers(), None)
							{
								true
							} else {
								false
							}
						}) {
							for (converter_archetype, hypothesis_reference) in
								&self.converter_in_play_hypotheses
							{
								if archetype == *converter_archetype {
									can_be_adjacent_converted_to_by
										.insert(archetype.clone(), hypothesis_reference);
								} else {
									can_have_conversion_stolen_by_adjacent
										.insert(archetype.clone(), hypothesis_reference);
								}
							}
						}
					}
					Affect::Puppet(_) => {
						for archetype in VillagerArchetype::iter().filter(|archetype| {
							if let Some(Affect::Puppet(_)) =
								archetype.affect(game_state.total_villagers(), None)
							{
								true
							} else {
								false
							}
						}) {
							for (converter_archetype, hypothesis_reference) in
								&self.converter_in_play_hypotheses
							{
								if archetype == *converter_archetype {
									can_be_adjacent_converted_to_by
										.insert(archetype.clone(), hypothesis_reference);
								} else {
									can_have_conversion_stolen_by_adjacent
										.insert(archetype.clone(), hypothesis_reference);
								}
							}
						}
					}
				}
			} else if let VillagerArchetype::GoodVillager(_) = &self.archetype {
				for (converter_archetype, hypothesis_reference) in
					&self.converter_in_play_hypotheses
				{
					can_be_converted_away_from_by_adjacent
						.insert(converter_archetype.clone(), hypothesis_reference);
				}
			}
		}

		if !in_deck && can_be_adjacent_converted_to_by.is_empty() {
			return repository.finalize(HypothesisResult::impossible());
		}

		// calculate the probability the archetype was in the initial draw
		let deck_count = game_state
			.deck()
			.iter()
			.filter(|archetype| **archetype == self.archetype)
			.count();

		let draw_count = match &self.archetype {
			VillagerArchetype::GoodVillager(_) => game_state.draw_stats().villagers(),
			VillagerArchetype::Outcast(_) => game_state.draw_stats().outcasts(),
			VillagerArchetype::Minion(_) => game_state.draw_stats().minions(),
			VillagerArchetype::Demon(_) => game_state.draw_stats().demons(),
		};

		let initial_draw_probability = draw_count as f64 / deck_count as f64;

		let any_conversions_in_play = !can_be_adjacent_converted_to_by.is_empty()
			|| !can_be_converted_away_from_by_adjacent.is_empty()
			|| !can_have_conversion_stolen_by_adjacent.is_empty();
		if !any_conversions_in_play {
			return repository.finalize(HypothesisResult::Conclusive(FitnessAndAction::new(
				initial_draw_probability,
				None,
			)));
		}

		let evaluator = repository.require_sub_evaluation(initial_draw_probability);

		evaluator.finalize(HypothesisResult::unimplemented())
	}
}
