mod hypotheses;
mod hypothesis;
pub mod player_action;

use demon_bluff_gameplay_engine::game_state::GameState;
use player_action::PlayerAction;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PredictionError {}

pub fn predict(state: &GameState) -> Result<PlayerAction, PredictionError> {
    todo!()
}
