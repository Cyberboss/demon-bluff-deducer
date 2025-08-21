use std::fmt::Display;

use demon_bluff_gameplay_engine::game_state::GameState;

use crate::hypothesis::{
    Hypothesis, HypothesisReference, HypothesisRegistrar, HypothesisRepository, HypothesisReturn,
    fittest_result,
};

use super::reveal::RevealHypothesis;

#[derive(Debug, PartialEq, Eq)]
pub struct MasterHypothesis {
    reveal_hypothesis: HypothesisReference,
    execute_hypothesis: HypothesisReference,
    ability_hypothesis: HypothesisReference,
}

impl MasterHypothesis {
    pub fn create(
        game_state: &GameState,
        mut registrar: HypothesisRegistrar,
    ) -> HypothesisReference {
        registrar.register(Self {
            reveal_hypothesis: RevealHypothesis::create(game_state, &mut registrar),
            execute_hypothesis: todo!(),
            ability_hypothesis: todo!(),
        })
    }
}

impl Display for MasterHypothesis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Master Hypothesis")
    }
}

impl Hypothesis for MasterHypothesis {
    fn evaluate(
        &mut self,
        _: &GameState,
        repository: &mut HypothesisRepository,
    ) -> HypothesisReturn {
        let evaluator = repository.require_sub_evaluation(0.0);
        let mut result = evaluator.sub_evaluate(&self.ability_hypothesis);
        result = fittest_result(evaluator.sub_evaluate(&self.reveal_hypothesis), result);
        result = fittest_result(evaluator.sub_evaluate(&self.execute_hypothesis), result);
        repository.create_return(result)
    }
}
