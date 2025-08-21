use std::{cell::RefCell, collections::HashSet, fmt::Display};

use demon_bluff_gameplay_engine::game_state::GameState;

use crate::player_action::PlayerAction;

pub struct HypothesisReference(usize);

pub struct HypothesisRepository {
    hypotheses: Vec<RefCell<Box<dyn Hypothesis>>>,
}

pub enum EvaluationRequestFulfillment {
    Pending,
    Ready(f64),
    BreakCycle,
}

pub trait Hypothesis {
    fn evaluate(
        &mut self,
        game_state: &GameState,
        repository: &mut HypothesisRepository,
    ) -> Option<f64>;

    fn action(&self) -> HashSet<PlayerAction>;
}

pub struct HypothesisRegistrar {
    hypotheses: Vec<RefCell<Box<dyn Hypothesis>>>,
}

impl HypothesisRepository {
    pub fn request_evaluation<F>(
        &mut self,
        current_fitness: f64,
        hypothesis_reference: &HypothesisReference,
        f: F,
    ) where
        F: FnMut(&mut dyn Hypothesis) -> EvaluationRequestFulfillment,
    {
        todo!()
    }
}

impl HypothesisRegistrar {
    pub fn register(&mut self, hypothesis: Box<dyn Hypothesis>) -> HypothesisReference {
        todo!()
    }
}
