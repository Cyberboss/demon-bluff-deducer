use demon_bluff_gameplay_engine::game_state::GameState;
use log::Log;

use crate::hypothesis::{
    Depth, FITNESS_UNKNOWN, Hypothesis, HypothesisBuilder, HypothesisReference,
    HypothesisRegistrar, HypothesisRepository, HypothesisReturn,
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
    ) -> HypothesisReturn
    where
        TLog: Log,
    {
        let mut evaluator = repository.require_sub_evaluation(FITNESS_UNKNOWN);

        let sub_result = evaluator.sub_evaluate(&self.sub_hypothesis);

        evaluator.create_return(sub_result)
    }
}
