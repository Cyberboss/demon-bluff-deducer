mod hypotheses;
mod hypothesis;
pub mod player_action;

use std::collections::HashSet;

use force_graph::ForceGraph;
use log::Log;
use thiserror::Error;

use demon_bluff_gameplay_engine::game_state::GameState;
use hypotheses::master::MasterHypothesis;
use hypothesis::{GraphNodeData, evaluate};
use player_action::PlayerAction;

use crate::hypotheses::master::MasterHypothesisBuilder;

#[derive(Error, Debug)]
pub enum PredictionError {
    #[error("Evaluation could not determine an action to perform!")]
    ConclusiveNoAction,
}

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
