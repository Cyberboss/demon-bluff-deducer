use demon_bluff_gameplay_engine::{game_state::GameState, villager::VillagerIndex};
use log::Log;

use crate::hypothesis::{
    Depth, FitnessAndAction, Hypothesis, HypothesisBuilder, HypothesisRegistrar,
    HypothesisRepository, HypothesisResult, HypothesisReturn,
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct NeedTestimonyHypothesisBuilder {
    index: VillagerIndex,
}

#[derive(Debug)]
pub struct NeedTestimonyHypothesis {
    index: VillagerIndex,
}

impl NeedTestimonyHypothesisBuilder {
    pub fn new(index: VillagerIndex) -> Self {
        Self { index }
    }
}

impl HypothesisBuilder for NeedTestimonyHypothesisBuilder {
    type HypothesisImpl = NeedTestimonyHypothesis;

    fn build<TLog>(
        self,
        game_state: &GameState,
        registrar: &mut HypothesisRegistrar<TLog>,
    ) -> Self::HypothesisImpl
    where
        TLog: ::log::Log,
    {
        Self::HypothesisImpl { index: self.index }
    }
}

impl Hypothesis for NeedTestimonyHypothesis {
    fn describe(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "Need testimony of {}", self.index)
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
