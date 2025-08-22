use std::collections::HashSet;

use demon_bluff_gameplay_engine::{game_state::GameState, villager::VillagerIndex};
use log::Log;

use crate::{
    hypothesis::{
        Depth, FitnessAndAction, Hypothesis, HypothesisReference, HypothesisRegistrar,
        HypothesisRepository, HypothesisResult, HypothesisReturn,
    },
    player_action::{AbilityAttempt, PlayerAction},
};

#[derive(Eq, PartialEq, Debug)]
pub struct AbilityHypothesis {}

impl AbilityHypothesis {
    pub fn create(_: &GameState, registrar: &mut HypothesisRegistrar) -> HypothesisReference {
        registrar.register(Self {})
    }
}

impl Hypothesis for AbilityHypothesis {
    fn describe(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "Ability Decision")
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
        repository.create_return(HypothesisResult::Conclusive(FitnessAndAction::impossible()))
    }
}
