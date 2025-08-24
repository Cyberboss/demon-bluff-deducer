use demon_bluff_gameplay_engine::game_state::GameState;
use log::Log;

use crate::{
    engine::{
        Depth, FitnessAndAction, Hypothesis, HypothesisBuilder, HypothesisRegistrar,
        HypothesisRepository, HypothesisResult, HypothesisReturn,
    },
    hypotheses::HypothesisType,
};

#[derive(Eq, PartialEq, Debug, Default, Clone)]
pub struct AbilityHypothesisBuilder {}

impl HypothesisBuilder for AbilityHypothesisBuilder {
    fn build<TLog>(self, _: &GameState, _: &mut HypothesisRegistrar<TLog>) -> HypothesisType
    where
        TLog: ::log::Log,
    {
        AbilityHypothesis {}.into()
    }
}

#[derive(Debug)]
pub struct AbilityHypothesis {}

impl Hypothesis for AbilityHypothesis {
    fn describe(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "Ability Decision")
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
        repository.create_return(HypothesisResult::unimplemented())
    }
}
