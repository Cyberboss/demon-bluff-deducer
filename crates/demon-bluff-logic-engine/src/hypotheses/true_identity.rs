use demon_bluff_gameplay_engine::{
    game_state::GameState,
    villager::{VillagerArchetype, VillagerIndex},
};
use log::Log;

use crate::hypothesis::{
    Depth, Hypothesis, HypothesisBuilder, HypothesisRegistrar, HypothesisRepository,
    HypothesisResult, HypothesisReturn,
};

use super::HypothesisType;

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct TrueIdentityHypothesisBuilder {
    index: VillagerIndex,
    archetype: VillagerArchetype,
}

#[derive(Debug)]
pub struct TrueIdentityHypothesis {
    index: VillagerIndex,
    archetype: VillagerArchetype,
}

impl TrueIdentityHypothesisBuilder {
    pub fn new(index: VillagerIndex, archetype: VillagerArchetype) -> Self {
        Self { index, archetype }
    }
}

impl HypothesisBuilder for TrueIdentityHypothesisBuilder {
    fn build<TLog>(self, _: &GameState, _: &mut HypothesisRegistrar<TLog>) -> HypothesisType
    where
        TLog: ::log::Log,
    {
        TrueIdentityHypothesis {
            index: self.index,
            archetype: self.archetype,
        }
        .into()
    }
}

impl Hypothesis for TrueIdentityHypothesis {
    fn describe(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{} is a {}", self.index, self.archetype)
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
