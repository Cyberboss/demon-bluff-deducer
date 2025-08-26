use demon_bluff_gameplay_engine::{
    game_state::GameState,
    villager::{VillagerArchetype, VillagerIndex},
};
use log::Log;

use crate::engine::{
    Depth, Hypothesis, HypothesisBuilder, HypothesisEvaluation, HypothesisRegistrar,
    HypothesisRepository, HypothesisResult,
};

use super::{HypothesisBuilderType, HypothesisType, desires::DesireType};

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
        game_state: &GameState,
        registrar: &mut impl HypothesisRegistrar<HypothesisBuilderType, DesireType>,
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

    fn evaluate(
        &mut self,
        log: &impl Log,
        depth: Depth,
        game_state: &GameState,
        repository: impl HypothesisRepository,
    ) -> HypothesisEvaluation {
        repository.finalize(HypothesisResult::unimplemented())
    }
}
