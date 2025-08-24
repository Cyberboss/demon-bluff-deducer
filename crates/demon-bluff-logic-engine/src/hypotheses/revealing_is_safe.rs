use demon_bluff_gameplay_engine::game_state::GameState;
use log::Log;

use crate::{
    hypotheses::HypothesisType,
    hypothesis::{
        Depth, FitnessAndAction, Hypothesis, HypothesisBuilder, HypothesisRegistrar,
        HypothesisRepository, HypothesisResult, HypothesisReturn,
    },
};

#[derive(Eq, PartialEq, Debug, Clone, Default)]
pub struct RevealingIsSafeHypothesisBuilder {}

#[derive(Debug)]
pub struct RevealingIsSafeHypothesis {}

impl HypothesisBuilder for RevealingIsSafeHypothesisBuilder {
    type HypothesisImpl = RevealingIsSafeHypothesis;

    fn build<TLog>(
        self,
        game_state: &GameState,
        registrar: &mut HypothesisRegistrar<TLog>,
    ) -> Self::HypothesisImpl
    where
        Self::HypothesisImpl: Hypothesis,
        HypothesisType: From<Self::HypothesisImpl>,
        TLog: ::log::Log,
    {
        Self::HypothesisImpl {}
    }
}

impl Hypothesis for RevealingIsSafeHypothesis {
    fn describe(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "Revealing Villagers is Safe")
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
