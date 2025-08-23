use demon_bluff_gameplay_engine::{
    game_state::GameState,
    villager::{Minion, VillagerArchetype},
};
use log::Log;

use crate::{
    hypotheses::archetype_in_play,
    hypothesis::{
        Depth, FitnessAndAction, Hypothesis, HypothesisReference, HypothesisRegistrar,
        HypothesisRepository, HypothesisResult, HypothesisReturn,
    },
};

#[derive(Eq, PartialEq, Debug)]
pub struct ArchetypeInPlayHypothesis {
    archetype: VillagerArchetype,
    counsellor_in_play_hypothesis: Option<HypothesisReference>,
}

impl ArchetypeInPlayHypothesis {
    pub fn create<TLog>(
        game_state: &GameState,
        registrar: &mut HypothesisRegistrar<TLog>,
        archetype: VillagerArchetype,
    ) -> HypothesisReference
    where
        TLog: Log,
    {
        let counsellor_in_play_hypothesis = match archetype {
            VillagerArchetype::GoodVillager(_) => Some(ArchetypeInPlayHypothesis::create(
                game_state,
                registrar,
                VillagerArchetype::Minion(Minion::Counsellor),
            )),
            _ => None,
        };

        registrar.register(Self {
            archetype,
            counsellor_in_play_hypothesis,
        })
    }
}

impl Hypothesis for ArchetypeInPlayHypothesis {
    fn describe(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{} in play", self.archetype)
    }

    fn evaluate<TLog>(
        &mut self,
        log: &TLog,
        depth: Depth,
        game_state: &GameState,
        repository: HypothesisRepository<TLog>,
    ) -> HypothesisReturn
    where
        TLog: Log,
    {
        repository.create_return(HypothesisResult::Conclusive(
            FitnessAndAction::unimplemented(),
        ))
    }
}
