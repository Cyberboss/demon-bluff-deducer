use demon_bluff_gameplay_engine::{game_state::GameState, villager::VillagerIndex};
use log::Log;

use crate::{
    hypotheses::{
        HypothesisType, corruption_in_play::CorruptionInPlayHypothesisBuilder,
        is_evil::IsEvilHypothesisBuilder, is_truthful::IsTruthfulHypothesisBuilder,
        negate::NegateHypothesisBuilder,
    },
    engine::{
        Depth, FITNESS_UNKNOWN, Hypothesis, HypothesisBuilder, HypothesisReference,
        HypothesisRegistrar, HypothesisRepository, HypothesisReturn, and_result,
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IsCorruptHypothesisBuilder {
    index: VillagerIndex,
}

/// Check if a given [`VillagerIndex`] is corrupt
#[derive(Debug)]
pub struct IsCorruptHypothesis {
    index: VillagerIndex,
    is_good_hypothesis: HypothesisReference,
    is_lying_hypothesis: HypothesisReference,
    corruption_in_play_hypothesis: HypothesisReference,
}

impl IsCorruptHypothesisBuilder {
    pub fn new(index: VillagerIndex) -> Self {
        Self { index }
    }
}

impl HypothesisBuilder for IsCorruptHypothesisBuilder {
    fn build<TLog>(self, _: &GameState, registrar: &mut HypothesisRegistrar<TLog>) -> HypothesisType
    where
        TLog: ::log::Log,
    {
        let is_good_hypothesis = registrar.register(NegateHypothesisBuilder::new(
            IsEvilHypothesisBuilder::new(self.index.clone()),
        ));
        let is_lying_hypothesis = registrar.register(NegateHypothesisBuilder::new(
            IsTruthfulHypothesisBuilder::new(self.index.clone()),
        ));
        let corruption_in_play_hypothesis =
            registrar.register(CorruptionInPlayHypothesisBuilder::default());

        IsCorruptHypothesis {
            index: self.index,
            is_lying_hypothesis,
            is_good_hypothesis,
            corruption_in_play_hypothesis,
        }
        .into()
    }
}

impl Hypothesis for IsCorruptHypothesis {
    fn describe(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{} is corrupt", self.index)
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

        let corruption_in_play_result = evaluator.sub_evaluate(&self.corruption_in_play_hypothesis);
        let is_good_result = evaluator.sub_evaluate(&self.is_good_hypothesis);
        let is_lying_result = evaluator.sub_evaluate(&self.is_lying_hypothesis);

        let result = and_result(
            corruption_in_play_result,
            and_result(is_good_result, is_lying_result),
        );

        evaluator.create_return(result)
    }
}
