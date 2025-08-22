use std::fmt::Display;

use demon_bluff_gameplay_engine::game_state::GameState;

use crate::{
    hypotheses::execute::ExecuteHypothesis,
    hypothesis::{
        Hypothesis, HypothesisReference, HypothesisRegistrar, HypothesisRepository,
        HypothesisReturn, fittest_result,
    },
};

use super::{ability::AbilityHypothesis, reveal::RevealHypothesis};

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
        let reveal_hypothesis = RevealHypothesis::create(game_state, &mut registrar);
        let execute_hypothesis = ExecuteHypothesis::create(game_state, &mut registrar);
        let ability_hypothesis = AbilityHypothesis::create(game_state, &mut registrar);
        registrar.register(Self {
            reveal_hypothesis,
            execute_hypothesis,
            ability_hypothesis,
        })
    }
}

impl Hypothesis for MasterHypothesis {
    fn describe(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "Master Hypothesis")
    }

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
