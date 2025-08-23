use std::fmt::Display;

use demon_bluff_gameplay_engine::game_state::GameState;
use log::Log;

use crate::{
    hypotheses::execute::ExecuteHypothesis,
    hypothesis::{
        Depth, Hypothesis, HypothesisReference, HypothesisRegistrar, HypothesisRepository,
        HypothesisReturn, or_result,
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
    pub fn create<TLog>(
        game_state: &GameState,
        mut registrar: &mut HypothesisRegistrar<TLog>,
    ) -> HypothesisReference
    where
        TLog: Log,
    {
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

    fn evaluate<TLog>(
        &mut self,
        log: &TLog,
        depth: Depth,
        game_state: &GameState,
        mut repository: HypothesisRepository<TLog>,
    ) -> HypothesisReturn
    where
        TLog: Log,
    {
        let mut evaluator = repository.require_sub_evaluation(0.0);
        let mut result = evaluator.sub_evaluate(&self.ability_hypothesis);
        result = or_result(evaluator.sub_evaluate(&self.reveal_hypothesis), result);
        result = or_result(evaluator.sub_evaluate(&self.execute_hypothesis), result);
        evaluator.create_return(result)
    }
}
