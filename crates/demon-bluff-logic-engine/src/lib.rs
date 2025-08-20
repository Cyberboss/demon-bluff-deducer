use demon_bluff_gameplay_engine::game_state::{Action, GameState};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PredictionError {}

pub fn predict(state: &GameState) -> Result<Action, PredictionError> {
    todo!()
}
