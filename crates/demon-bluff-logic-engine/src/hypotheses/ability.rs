use demon_bluff_gameplay_engine::game_state::GameState;

use crate::hypothesis::{
    Hypothesis, HypothesisReference, HypothesisRegistrar, HypothesisRepository, HypothesisReturn,
};

#[derive(Eq, PartialEq, Debug)]
pub struct AbilityHypothesis {}

impl AbilityHypothesis {
    pub fn create(_: &GameState, registrar: &mut HypothesisRegistrar) -> HypothesisReference {
        registrar.register(Self {})
    }
}

impl Hypothesis for AbilityHypothesis {
    fn evaluate(
        &mut self,
        game_state: &GameState,
        repository: &mut HypothesisRepository,
    ) -> HypothesisReturn {
        todo!()
    }
}
