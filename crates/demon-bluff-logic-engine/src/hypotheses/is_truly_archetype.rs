use demon_bluff_gameplay_engine::{
    game_state::GameState,
    villager::{VillagerArchetype, VillagerIndex},
};
use log::Log;

use crate::engine::{
    Depth, Hypothesis, HypothesisBuilder, HypothesisRegistrar, HypothesisRepository,
    HypothesisResult, HypothesisReturn,
};

use super::HypothesisType;

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct IsTrulyArchetypeHypothesisBuilder {
    archetype: VillagerArchetype,
    index: VillagerIndex,
}

impl IsTrulyArchetypeHypothesisBuilder {
    pub fn new(archetype: VillagerArchetype, index: VillagerIndex) -> Self {
        Self { archetype, index }
    }
}

#[derive(Debug)]
pub struct IsTrulyArchetypeHypothesis {
    archetype: VillagerArchetype,
    index: VillagerIndex,
}

impl HypothesisBuilder for IsTrulyArchetypeHypothesisBuilder {
    fn build<TLog>(self, _: &GameState, registrar: &mut HypothesisRegistrar<TLog>) -> HypothesisType
    where
        TLog: ::log::Log,
    {
        IsTrulyArchetypeHypothesis {
            archetype: self.archetype,
            index: self.index,
        }
        .into()
    }
}

impl Hypothesis for IsTrulyArchetypeHypothesis {
    fn describe(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{} is a {}", self.index, self.archetype)
    }

    fn wip(&self) -> bool {
        true
    }

    fn evaluate<TLog>(
        &mut self,
        _: &TLog,
        _: Depth,
        _: &GameState,
        repository: HypothesisRepository<TLog>,
    ) -> HypothesisReturn
    where
        TLog: Log,
    {
        repository.create_return(HypothesisResult::unimplemented())
    }
}
