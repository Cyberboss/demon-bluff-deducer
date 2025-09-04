use demon_bluff_gameplay_engine::{
	game_state::GameState,
	villager::{GoodVillager, Outcast, Villager, VillagerArchetype, VillagerIndex},
};
use log::Log;

use super::{HypothesisBuilderType, HypothesisType, desires::DesireType};
use crate::{
	Breakpoint,
	engine::{
		Depth, Hypothesis, HypothesisBuilder, HypothesisEvaluation, HypothesisFunctions,
		HypothesisRegistrar, HypothesisRepository, HypothesisResult,
	},
};

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct AbilityIndexHypothesisBuilder {
	index: VillagerIndex,
}

#[derive(Debug)]
pub struct AbilityIndexHypothesis {
	index: VillagerIndex,
}

impl AbilityIndexHypothesisBuilder {
	pub fn new(index: VillagerIndex) -> Self {
		Self { index }
	}
}

impl HypothesisBuilder for AbilityIndexHypothesisBuilder {
	fn build(
		self,
		game_state: &GameState,
		registrar: &mut impl HypothesisRegistrar<HypothesisBuilderType, DesireType>,
	) -> HypothesisType {
		AbilityIndexHypothesis { index: self.index }.into()
	}
}

impl Hypothesis for AbilityIndexHypothesis {
	fn describe(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
		write!(f, "Use {}'s ability", self.index)
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
		let archetype = match game_state.villager(&self.index) {
			Villager::Hidden(_) => panic!("We shouldn't be evaluating a hidden villager!"),
			Villager::Active(active_villager) => active_villager.instance().archetype(),
			Villager::Confirmed(confirmed_villager) => confirmed_villager.instance().archetype(),
		};

		let result = match archetype {
			VillagerArchetype::GoodVillager(good_villager) => match good_villager {
				GoodVillager::Dreamer => HypothesisResult::unimplemented(),
				GoodVillager::Druid => HypothesisResult::unimplemented(),
				GoodVillager::FortuneTeller => HypothesisResult::unimplemented(),
				GoodVillager::Jester => HypothesisResult::unimplemented(),
				GoodVillager::Judge => HypothesisResult::unimplemented(),
				GoodVillager::Slayer => HypothesisResult::unimplemented(),
				GoodVillager::Scout
				| GoodVillager::Bard
				| GoodVillager::Alchemist
				| GoodVillager::Architect
				| GoodVillager::Baker
				| GoodVillager::Bishop
				| GoodVillager::Confessor
				| GoodVillager::Empress
				| GoodVillager::Witness
				| GoodVillager::Enlightened
				| GoodVillager::Gemcrafter
				| GoodVillager::Hunter
				| GoodVillager::Knight
				| GoodVillager::Knitter
				| GoodVillager::Lover
				| GoodVillager::Medium
				| GoodVillager::Oracle
				| GoodVillager::Poet => {
					panic!("This good villager archetype shouldn't have an ability!?")
				}
			},
			VillagerArchetype::Outcast(outcast) => match outcast {
				Outcast::Drunk | Outcast::Wretch | Outcast::Bombardier | Outcast::Doppelganger => {
					panic!("This outcast archetype shouldn't have an ability!?")
				}
				Outcast::PlagueDoctor => todo!(),
			},
			VillagerArchetype::Minion(_) | VillagerArchetype::Demon(_) => {
				panic!("There shouldn't be an active minion or demon??")
			}
		};

		repository.finalize(result)
	}
}
