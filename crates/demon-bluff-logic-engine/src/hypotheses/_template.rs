use demon_bluff_gameplay_engine::game_state::GameState;
use log::Log;

use crate::engine::{
    Depth, FITNESS_UNKNOWN, Hypothesis, HypothesisBuilder, HypothesisEvaluation,
    HypothesisReference, HypothesisRegistrar, HypothesisRepository,
};

use super::HypothesisType;

#[derive(Eq, PartialEq, Debug, Clone, Default)]
pub struct TemplateHypothesisBuilder {}

#[derive(Debug)]
pub struct TemplateHypothesis {
    sub_hypothesis: HypothesisReference,
}

impl HypothesisBuilder for TemplateHypothesisBuilder {
    fn build<TLog>(self, _: &GameState, registrar: &mut HypothesisRegistrar<TLog>) -> HypothesisType
    where
        TLog: ::log::Log,
    {
        TemplateHypothesis {
            sub_hypothesis: registrar.register(TemplateHypothesisBuilder {}),
        }
        .into()
    }
}

impl Hypothesis for TemplateHypothesis {
    fn describe(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "REPLACE ME")
    }

    fn wip(&self) -> bool {
        true
    }

    fn evaluate<TLog>(
        &mut self,
        _: &TLog,
        _: Depth,
        _: &GameState,
        repository: HypothesisRepository<TLog>,
    ) -> HypothesisEvaluation
    where
        TLog: Log,
    {
        let mut evaluator = repository.require_sub_evaluation(FITNESS_UNKNOWN);

        let sub_result = evaluator.sub_evaluate(&self.sub_hypothesis);

        evaluator.finalize(sub_result)
    }
}
