use demon_bluff_gameplay_engine::{
    game_state::GameState,
    villager::{VillagerArchetype, VillagerIndex},
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
    fn build(
        self,
        _: &GameState,
        _: &mut impl HypothesisRegistrar<HypothesisBuilderType, DesireType>,
    ) -> HypothesisType {
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

    fn evaluate<TLog, FDebugBreak>(
        &mut self,
        _: &TLog,
        _: Depth,
        _: &GameState,
        repository: HypothesisRepository<TLog, FDebugBreak>,
    ) -> HypothesisEvaluation
    where
        TLog: Log,
        FDebugBreak: FnMut(Breakpoint) + Clone,
    {
        repository.finalize(HypothesisResult::unimplemented())
    }
}
