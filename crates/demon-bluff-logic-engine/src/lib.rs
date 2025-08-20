use std::collections::HashSet;

use demon_bluff_gameplay_engine::{game_state::GameState, villager::VillagerIndex};
use thiserror::Error;

#[derive(Debug, Eq)]
pub struct AbilityAttempt {
    source: VillagerIndex,
    targets: HashSet<VillagerIndex>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum PlayerAction {
    TryReveal(VillagerIndex),
    TryExecute(VillagerIndex),
    Ability(AbilityAttempt),
}

#[derive(Error, Debug)]
pub enum PredictionError {}

impl PartialEq for AbilityAttempt {
    fn eq(&self, other: &Self) -> bool {
        self.source == other.source
            && self.targets.len() == other.targets.len()
            && self
                .targets
                .iter()
                .all(|target| other.targets.contains(target))
    }
}

pub fn predict(state: &GameState) -> Result<PlayerAction, PredictionError> {
    todo!()
}
