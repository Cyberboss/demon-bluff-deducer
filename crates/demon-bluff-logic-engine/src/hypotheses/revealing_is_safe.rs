use demon_bluff_gameplay_engine::game_state::GameState;
use log::Log;

use crate::hypothesis::{
    Depth, FitnessAndAction, Hypothesis, HypothesisReference, HypothesisRegistrar,
    HypothesisRepository, HypothesisResult, HypothesisReturn,
};

#[derive(Eq, PartialEq, Debug)]
pub struct RevealingIsSafeHypothesis {}

impl RevealingIsSafeHypothesis {
    pub fn create<TLog>(
        _: &GameState,
        registrar: &mut HypothesisRegistrar<TLog>,
    ) -> HypothesisReference
    where
        TLog: Log,
    {
        registrar.register(Self {})
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
