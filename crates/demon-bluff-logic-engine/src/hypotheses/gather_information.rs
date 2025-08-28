use demon_bluff_gameplay_engine::game_state::GameState;
use log::Log;

use super::{DesireType, HypothesisBuilderType};
use crate::{
    Breakpoint,
    engine::{
        Depth, Hypothesis, HypothesisBuilder, HypothesisEvaluation, HypothesisEvaluator,
        HypothesisFunctions, HypothesisReference, HypothesisRegistrar, HypothesisRepository,
        or_result,
    },
    hypotheses::{
        HypothesisType, ability::AbilityHypothesisBuilder, reveal::RevealHypothesisBuilder,
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
    fn build(
        self,
        game_state: &GameState,
        registrar: &mut impl HypothesisRegistrar<HypothesisBuilderType, DesireType>,
    ) -> HypothesisType {
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

    fn evaluate<TLog, FDebugBreak>(
        &mut self,
        _: &TLog,
        _: Depth,
        _: &GameState,
        repository: HypothesisRepository<TLog, FDebugBreak>,
    ) -> HypothesisEvaluation
    where
        TLog: Log,
        FDebugBreak: FnMut(Breakpoint) + Clone,
    {
        let mut evaluator = repository.require_sub_evaluation(0.0);

        let result = or_result(
            evaluator.sub_evaluate(&self.ability_hypothesis),
            evaluator.sub_evaluate(&self.reveal_hypothesis),
        );

        evaluator.finalize(result)
    }
}
