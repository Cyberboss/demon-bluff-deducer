use demon_bluff_gameplay_engine::game_state::GameState;
use log::Log;

use crate::{
    hypotheses::HypothesisType,
    hypothesis::{
        Depth, Hypothesis, HypothesisBuilder, HypothesisReference, HypothesisRegistrar,
        HypothesisRepository, HypothesisResult, HypothesisReturn,
    },
};

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct NegateHypothesisBuilder {
    target_hypothesis_builder: Box<HypothesisType>,
}

#[derive(Debug)]
pub struct NegateHypothesis {
    target_hypothesis: HypothesisReference,
}

impl NegateHypothesisBuilder {
    pub fn new<TBuilder>(builder: TBuilder) -> Self
    where
        TBuilder: HypothesisBuilder,
        HypothesisType: From<TBuilder>,
    {
        Self {
            target_hypothesis_builder: Box::new(builder.into()),
        }
    }
}

impl HypothesisBuilder for NegateHypothesisBuilder {
    type HypothesisImpl = NegateHypothesis;

    fn build<TLog>(
        self,
        game_state: &GameState,
        registrar: &mut HypothesisRegistrar<TLog>,
    ) -> Self::HypothesisImpl
    where
        TLog: ::log::Log,
    {
        let target_hypothesis = registrar.register(self.target_hypothesis_builder);
        Self::HypothesisImpl { target_hypothesis }
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
    ) -> HypothesisReturn
    where
        TLog: Log,
    {
        let mut evaluator = repository.require_sub_evaluation(0.0);

        let result = evaluator.sub_evaluate(&self.target_hypothesis);

        evaluator.create_return(match result {
            HypothesisResult::Pending(fitness_and_action) => {
                HypothesisResult::Pending(fitness_and_action.invert())
            }
            HypothesisResult::Conclusive(fitness_and_action) => {
                HypothesisResult::Conclusive(fitness_and_action.invert())
            }
        })
    }
}
