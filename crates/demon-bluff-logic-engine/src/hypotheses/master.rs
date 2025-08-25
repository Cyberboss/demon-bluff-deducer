use demon_bluff_gameplay_engine::game_state::GameState;
use log::Log;

use crate::{
    engine_old::{
        Depth, Hypothesis, HypothesisBuilder, HypothesisReference, HypothesisRegistrar,
        HypothesisRepository, HypothesisResult, HypothesisReturn, decide_result,
    },
    hypotheses::{
        HypothesisType, execute::ExecuteHypothesisBuilder,
        gather_information::GatherInformationHypothesisBuilder,
    },
};

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct MasterHypothesisBuilder {}

impl HypothesisBuilder for MasterHypothesisBuilder {
    fn build<TLog>(self, _: &GameState, registrar: &mut HypothesisRegistrar<TLog>) -> HypothesisType
    where
        TLog: ::log::Log,
    {
        let execute_hypothesis = registrar.register(ExecuteHypothesisBuilder::default());
        let info_hypothesis = registrar.register(GatherInformationHypothesisBuilder::default());
        MasterHypothesis {
            execute_hypothesis,
            info_hypothesis,
        }
        .into()
    }
}

#[derive(Debug)]
pub struct MasterHypothesis {
    info_hypothesis: HypothesisReference,
    execute_hypothesis: HypothesisReference,
}

impl Hypothesis for MasterHypothesis {
    fn describe(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "Master Hypothesis")
    }

    fn evaluate<TLog>(
        &mut self,
        _: &TLog,
        _: Depth,
        _: &GameState,
        repository: HypothesisRepository<TLog>,
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
        result = decide_result(evaluator.sub_evaluate(&self.info_hypothesis), result);
        evaluator.create_return(result)
    }
}
