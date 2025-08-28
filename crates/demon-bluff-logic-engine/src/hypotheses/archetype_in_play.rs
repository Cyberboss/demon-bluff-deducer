use demon_bluff_gameplay_engine::{
	affect::Affect,
	game_state::GameState,
	villager::{Minion, VillagerArchetype},
};
use log::Log;

use super::{DesireType, HypothesisBuilderType};
use crate::{
	Breakpoint,
	engine::{
		Depth, Hypothesis, HypothesisBuilder, HypothesisEvaluation, HypothesisFunctions,
		HypothesisReference, HypothesisRegistrar, HypothesisRepository, HypothesisResult,
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
	counsellor_in_play_hypothesis: Option<HypothesisReference>,
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
		let counsellor_in_play_hypothesis = match self.archetype {
			VillagerArchetype::GoodVillager(_) => {
				Some(registrar.register(ArchetypeInPlayHypothesisBuilder::new(
					VillagerArchetype::Minion(Minion::Counsellor),
				)))
			}
			_ => None,
		};

		ArchetypeInPlayHypothesis {
			archetype: self.archetype,
			counsellor_in_play_hypothesis,
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
		// step one, eliminate the possibility if it's not in the deck
		// currently the only case where an archetype not in the deck can appear is the puppeteer
		let mut can_be_converted = false;
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
					Affect::Puppet(_) => can_be_converted = true,
				}
			}
		}

		if !in_deck && !can_be_converted {
			return repository.finalize(HypothesisResult::impossible());
		}

		repository.finalize(HypothesisResult::unimplemented())
	}
}
