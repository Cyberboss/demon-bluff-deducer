use demon_bluff_gameplay_engine::game_state::GameState;
use log::Log;

use crate::{
    hypotheses::{
        HypothesisType, execute::ExecuteHypothesisBuilder,
        gather_information::GatherInformationHypothesisBuilder,
    },
    hypothesis::{
        Depth, Hypothesis, HypothesisBuilder, HypothesisReference, HypothesisRegistrar,
        HypothesisRepository, HypothesisResult, HypothesisReturn, or_result,
    },
};

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct MasterHypothesisBuilder {}

impl HypothesisBuilder for MasterHypothesisBuilder {
    type HypothesisImpl = MasterHypothesis;

    fn build<TLog>(
        self,
        _: &GameState,
        registrar: &mut HypothesisRegistrar<TLog>,
    ) -> Self::HypothesisImpl
    where
        Self::HypothesisImpl: Hypothesis,
        HypothesisType: From<Self::HypothesisImpl>,
        TLog: ::log::Log,
    {
        let execute_hypothesis = registrar.register(ExecuteHypothesisBuilder::default());
        let info_hypothesis = registrar.register(GatherInformationHypothesisBuilder::default());
        Self::HypothesisImpl {
            execute_hypothesis,
            info_hypothesis,
        }
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
        result = or_result(evaluator.sub_evaluate(&self.execute_hypothesis), result);
        evaluator.create_return(result)
    }
}
