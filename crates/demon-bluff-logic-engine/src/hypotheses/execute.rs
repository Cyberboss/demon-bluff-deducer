use demon_bluff_gameplay_engine::game_state::GameState;

use crate::hypothesis::{
    Hypothesis, HypothesisReference, HypothesisRegistrar, HypothesisRepository, HypothesisReturn,
};

#[derive(Eq, PartialEq, Debug)]
pub struct ExecuteHypothesis {}

impl ExecuteHypothesis {
    pub fn create(_: &GameState, registrar: &mut HypothesisRegistrar) -> HypothesisReference {
        registrar.register(Self {})
    }
}

impl Hypothesis for ExecuteHypothesis {
    fn describe(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "Execution Decision")
    }

    fn evaluate(
        &mut self,
        game_state: &GameState,
        repository: &mut HypothesisRepository,
    ) -> HypothesisReturn {
        todo!()
    }
}
