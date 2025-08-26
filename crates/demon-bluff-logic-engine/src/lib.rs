#![feature(breakpoint)]

mod debugger;
mod engine;
mod hypotheses;
mod player_action;
mod prediction_error;

use std::collections::HashSet;

use engine::evaluate;
use log::Log;

pub use self::{
    debugger::Debugger, player_action::PlayerAction, prediction_error::PredictionError,
};
use demon_bluff_gameplay_engine::game_state::GameState;

use crate::hypotheses::MasterHypothesisBuilder;

pub fn predict_with_debugger<F>(
    log: &impl Log,
    state: &GameState,
    debugger: F,
) -> Result<HashSet<PlayerAction>, PredictionError>
where
    F: FnMut(&mut Debugger),
{
    evaluate(
        state,
        MasterHypothesisBuilder::default(),
        log,
        Some(debugger),
    )
}

pub fn predict(
    log: &impl Log,
    state: &GameState,
) -> Result<HashSet<PlayerAction>, PredictionError> {
    evaluate(
        state,
        MasterHypothesisBuilder::default(),
        log,
        None::<fn(&mut Debugger)>,
    )
}
