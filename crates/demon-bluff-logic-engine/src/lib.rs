#![feature(breakpoint)]

mod engine;
mod hypotheses;
mod player_action;
mod prediction_error;

use std::collections::HashSet;

use demon_bluff_gameplay_engine::game_state::GameState;
use engine::evaluate;
use log::Log;

pub use self::{
    engine::{Breakpoint, DebuggerContext, DesireNode, HypothesisNode, Node},
    player_action::PlayerAction,
    prediction_error::PredictionError,
};
use crate::hypotheses::MasterHypothesisBuilder;

pub fn predict_with_debugger<FDebugBreak>(
    log: &impl Log,
    state: &GameState,
    breakpoint_handler: FDebugBreak,
) -> Result<HashSet<PlayerAction>, PredictionError>
where
    FDebugBreak: FnMut(Breakpoint) + Clone,
{
    evaluate(
        state,
        MasterHypothesisBuilder::default(),
        log,
        Some(breakpoint_handler),
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
        None::<fn(Breakpoint)>,
    )
}
