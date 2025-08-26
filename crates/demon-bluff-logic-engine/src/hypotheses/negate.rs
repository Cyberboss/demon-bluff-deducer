use demon_bluff_gameplay_engine::game_state::GameState;
use log::Log;

use crate::{
    engine::{
        Depth, FITNESS_UNKNOWN, Hypothesis, HypothesisBuilder, HypothesisEvaluation,
        HypothesisEvaluator, HypothesisFunctions, HypothesisReference, HypothesisRegistrar,
        HypothesisRepository, HypothesisResult,
    },
    hypotheses::{HypothesisBuilderType, HypothesisType},
};

use super::DesireType;

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
        game_state: &GameState,
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

    fn evaluate(
        &mut self,
        log: &impl Log,
        depth: Depth,
        game_state: &GameState,
        repository: impl HypothesisRepository,
    ) -> HypothesisEvaluation {
        let mut evaluator = repository.require_sub_evaluation(FITNESS_UNKNOWN);

        let result = evaluator
            .sub_evaluate(&self.target_hypothesis)
            .map(|fitness_and_action| fitness_and_action.invert());

        evaluator.finalize(result)
    }
}
