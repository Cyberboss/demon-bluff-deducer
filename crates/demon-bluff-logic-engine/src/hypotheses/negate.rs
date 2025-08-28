use demon_bluff_gameplay_engine::game_state::GameState;
use log::Log;

use super::DesireType;
use crate::{
    engine::{
        Depth, FITNESS_UNKNOWN, Hypothesis, HypothesisBuilder, HypothesisEvaluation,
        HypothesisEvaluator, HypothesisFunctions, HypothesisReference, HypothesisRegistrar,
        HypothesisRepository,
    },
    hypotheses::{HypothesisBuilderType, HypothesisType},
};

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct NegateHypothesisBuilder {
    target_hypothesis_builder: Box<HypothesisBuilderType>,
}

#[derive(Debug)]
pub struct NegateHypothesis {
    target_hypothesis: HypothesisReference,
}

impl NegateHypothesisBuilder {
    pub fn new<TBuilder>(builder: TBuilder) -> Self
    where
        TBuilder: HypothesisBuilder,
        HypothesisBuilderType: From<TBuilder>,
    {
        Self {
            target_hypothesis_builder: Box::new(builder.into()),
        }
    }
}

impl HypothesisBuilder for NegateHypothesisBuilder {
    fn build(
        self,
        _: &GameState,
        registrar: &mut impl HypothesisRegistrar<HypothesisBuilderType, DesireType>,
    ) -> HypothesisType {
        let target_hypothesis = registrar.register_builder_type(*self.target_hypothesis_builder);
        NegateHypothesis { target_hypothesis }.into()
    }
}

impl Hypothesis for NegateHypothesis {
    fn describe(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "Negate {}", self.target_hypothesis)
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

        let result = evaluator
            .sub_evaluate(&self.target_hypothesis)
            .map(|fitness_and_action| fitness_and_action.invert());

        evaluator.finalize(result)
    }
}
