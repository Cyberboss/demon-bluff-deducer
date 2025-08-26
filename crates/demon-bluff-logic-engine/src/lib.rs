#![feature(breakpoint)]

mod engine;
mod hypotheses;
pub mod player_action;
mod prediction_error;

use std::collections::HashSet;

use engine::{GraphNodeData, evaluate};
use force_graph::ForceGraph;
use log::Log;

use demon_bluff_gameplay_engine::game_state::GameState;
use player_action::PlayerAction;
use prediction_error::PredictionError;

use crate::hypotheses::MasterHypothesisBuilder;

pub fn predict_with_graph<F>(
    log: &impl Log,
    state: &GameState,
    graph_stepper: F,
) -> Result<HashSet<PlayerAction>, PredictionError>
where
    F: FnMut(&mut ForceGraph<GraphNodeData>),
{
    evaluate(
        state,
        MasterHypothesisBuilder::default(),
        log,
        Some(graph_stepper),
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
        None::<fn(&mut ForceGraph<GraphNodeData>)>,
    )
}
