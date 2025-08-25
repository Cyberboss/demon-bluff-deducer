use demon_bluff_gameplay_engine::{
    game_state::{self, GameState},
    villager::{GoodVillager, Minion, Outcast, Villager, VillagerArchetype, VillagerIndex},
};
use log::Log;

use crate::engine::{
    Depth, FITNESS_UNKNOWN, Hypothesis, HypothesisBuilder, HypothesisReference,
    HypothesisRegistrar, HypothesisRepository, HypothesisResult, HypothesisReturn,
};

use super::HypothesisType;

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
    fn build<TLog>(self, _: &GameState, registrar: &mut HypothesisRegistrar<TLog>) -> HypothesisType
    where
        TLog: ::log::Log,
    {
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

    fn evaluate<TLog>(
        &mut self,
        _: &TLog,
        _: Depth,
        game_state: &GameState,
        repository: HypothesisRepository<TLog>,
    ) -> HypothesisReturn
    where
        TLog: Log,
    {
        let archetype = match game_state.villager(&self.index) {
            Villager::Hidden(_) => panic!("We shouldn't be evaluating a hidden villager!"),
            Villager::Active(active_villager) => active_villager.instance().archetype(),
            Villager::Confirmed(confirmed_villager) => confirmed_villager.instance().archetype(),
        };

        match archetype {
            VillagerArchetype::GoodVillager(good_villager) => match good_villager {
                GoodVillager::Dreamer => todo!(),
                GoodVillager::Druid => todo!(),
                GoodVillager::FortuneTeller => todo!(),
                GoodVillager::Jester => todo!(),
                GoodVillager::Judge => todo!(),
                GoodVillager::Slayer => todo!(),
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
        }

        repository.create_return(HypothesisResult::unimplemented())
    }
}
