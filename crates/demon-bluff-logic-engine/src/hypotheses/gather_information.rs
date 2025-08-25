use demon_bluff_gameplay_engine::game_state::GameState;
use log::Log;

use crate::{
    hypotheses::{
        HypothesisType, ability::AbilityHypothesisBuilder, reveal::RevealHypothesisBuilder,
    },
    engine_old::{
        Depth, Hypothesis, HypothesisBuilder, HypothesisReference, HypothesisRepository,
        HypothesisReturn, or_result,
    },
};

#[derive(PartialEq, Eq, Clone, Default, Debug)]
pub struct GatherInformationHypothesisBuilder {}

#[derive(Debug)]
pub struct GatherInformationHypothesis {
    reveal_hypothesis: HypothesisReference,
    ability_hypothesis: HypothesisReference,
}

impl HypothesisBuilder for GatherInformationHypothesisBuilder {
    fn build<TLog>(
        self,
        _: &::demon_bluff_gameplay_engine::game_state::GameState,
        registrar: &mut crate::engine_old::HypothesisRegistrar<TLog>,
    ) -> HypothesisType
    where
        TLog: ::log::Log,
    {
        let reveal_hypothesis = registrar.register(RevealHypothesisBuilder::default());
        let ability_hypothesis = registrar.register(AbilityHypothesisBuilder::default());
        GatherInformationHypothesis {
            reveal_hypothesis,
            ability_hypothesis,
        }
        .into()
    }
}

impl Hypothesis for GatherInformationHypothesis {
    fn describe(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "Gather Information")
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

        let result = or_result(
            evaluator.sub_evaluate(&self.ability_hypothesis),
            evaluator.sub_evaluate(&self.reveal_hypothesis),
        );

        evaluator.create_return(result)
    }
}
