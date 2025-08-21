mod hypotheses;
mod hypothesis;
pub mod player_action;

use std::collections::HashSet;

use demon_bluff_gameplay_engine::{
    game_state::GameState,
    villager::{GoodVillager, Villager, VillagerArchetype, VillagerIndex},
};
use player_action::PlayerAction;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PredictionError {}

pub fn predict(state: &GameState) -> Result<PlayerAction, PredictionError> {
    let registry = initialize_hypotheses();

    todo!()
}
