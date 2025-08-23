use demon_bluff_gameplay_engine::{game_state::GameState, villager::VillagerIndex};
use log::Log;

use crate::{
    hypotheses::{
        corruption_in_play::CorruptionInPlayHypothesis, is_evil::IsEvilHypothesis,
        is_truthful::IsTruthfulHypothesis, negate::NegateHypothesis,
    },
    hypothesis::{
        Depth, FITNESS_UNKNOWN, Hypothesis, HypothesisReference, HypothesisRegistrar,
        HypothesisRepository, HypothesisReturn, and_result,
    },
};

/// Check if a given [`VillagerIndex`] is corrupt
#[derive(Eq, PartialEq, Debug)]
pub struct IsCorruptHypothesis {
    index: VillagerIndex,
    is_good_hypothesis: HypothesisReference,
    is_lying_hypothesis: HypothesisReference,
    corruption_in_play_hypothesis: HypothesisReference,
}

impl IsCorruptHypothesis {
    pub fn create<TLog>(
        game_state: &GameState,
        mut registrar: &mut HypothesisRegistrar<TLog>,
        index: VillagerIndex,
    ) -> HypothesisReference
    where
        TLog: Log,
    {
        let is_evil_hypothesis =
            IsEvilHypothesis::create(game_state, &mut registrar, index.clone());
        let is_good_hypothesis =
            NegateHypothesis::create(game_state, &mut registrar, is_evil_hypothesis);
        let is_truthful_hypothesis =
            IsTruthfulHypothesis::create(game_state, &mut registrar, index.clone());
        let is_lying_hypothesis =
            NegateHypothesis::create(game_state, &mut registrar, is_truthful_hypothesis);
        let corruption_in_play_hypothesis =
            CorruptionInPlayHypothesis::create(game_state, &mut registrar);
        registrar.register(Self {
            index,
            is_good_hypothesis,
            is_lying_hypothesis,
            corruption_in_play_hypothesis,
        })
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
