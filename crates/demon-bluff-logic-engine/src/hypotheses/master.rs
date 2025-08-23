use std::fmt::Display;

use demon_bluff_gameplay_engine::game_state::GameState;
use log::Log;

use crate::{
    hypotheses::{execute::ExecuteHypothesis, gather_information::GatherInformationHypothesis},
    hypothesis::{
        Depth, Hypothesis, HypothesisReference, HypothesisRegistrar, HypothesisRepository,
        HypothesisResult, HypothesisReturn, or_result,
    },
};

#[derive(Debug, PartialEq, Eq)]
pub struct MasterHypothesis {
    info_hypothesis: HypothesisReference,
    execute_hypothesis: HypothesisReference,
}

impl MasterHypothesis {
    pub fn create<TLog>(
        game_state: &GameState,
        mut registrar: &mut HypothesisRegistrar<TLog>,
    ) -> HypothesisReference
    where
        TLog: Log,
    {
        let info_hypothesis = GatherInformationHypothesis::create(game_state, &mut registrar);
        let execute_hypothesis = ExecuteHypothesis::create(game_state, &mut registrar);
        registrar.register(Self {
            info_hypothesis,
            execute_hypothesis,
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
        let mut result = evaluator.sub_evaluate(&self.execute_hypothesis);
        match &result {
            HypothesisResult::Pending(_) => {}
            HypothesisResult::Conclusive(fitness_and_action) => {
                if fitness_and_action.is_certain() {
                    return evaluator
                        .create_return(HypothesisResult::Conclusive(fitness_and_action.clone()));
                }
            }
        }
        result = or_result(evaluator.sub_evaluate(&self.execute_hypothesis), result);
        evaluator.create_return(result)
    }
}
