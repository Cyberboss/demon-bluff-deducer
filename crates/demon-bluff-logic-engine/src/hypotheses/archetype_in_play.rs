use demon_bluff_gameplay_engine::{
    game_state::GameState,
    villager::{Minion, VillagerArchetype},
};
use log::Log;

use crate::{
    hypotheses::HypothesisType,
    hypothesis::{
        Depth, FitnessAndAction, Hypothesis, HypothesisBuilder, HypothesisReference,
        HypothesisRegistrar, HypothesisRepository, HypothesisResult, HypothesisReturn,
    },
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
    fn build<TLog>(self, _: &GameState, registrar: &mut HypothesisRegistrar<TLog>) -> HypothesisType
    where
        TLog: Log,
    {
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
