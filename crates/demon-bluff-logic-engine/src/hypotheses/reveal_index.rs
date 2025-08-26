use demon_bluff_gameplay_engine::{game_state::GameState, villager::VillagerIndex};
use log::Log;

use crate::{
    engine::{
        Depth, FitnessAndAction, Hypothesis, HypothesisBuilder, HypothesisEvaluation,
        HypothesisEvaluator, HypothesisFunctions, HypothesisReference, HypothesisRegistrar,
        HypothesisRepository, HypothesisResult, or_result,
    },
    hypotheses::{
        HypothesisType, need_testimony::NeedTestimonyHypothesisBuilder,
        revealing_is_safe::RevealingIsSafeHypothesisBuilder,
    },
    player_action::PlayerAction,
};

use super::{DesireType, HypothesisBuilderType};

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct RevealIndexHypothesisBuilder {
    index: VillagerIndex,
}

#[derive(Debug)]
pub struct RevealIndexHypothesis {
    index: VillagerIndex,
    revealing_is_safe_hypothesis: HypothesisReference,
    need_testimony_hypothesis: HypothesisReference,
}

impl RevealIndexHypothesisBuilder {
    pub fn new(index: VillagerIndex) -> Self {
        Self { index }
    }
}

impl HypothesisBuilder for RevealIndexHypothesisBuilder {
    fn build(
        self,
        _: &GameState,
        registrar: &mut impl HypothesisRegistrar<HypothesisBuilderType, DesireType>,
    ) -> HypothesisType {
        let revealing_is_safe_hypothesis =
            registrar.register(RevealingIsSafeHypothesisBuilder::default());
        let need_testimony_hypothesis =
            registrar.register(NeedTestimonyHypothesisBuilder::new(self.index.clone()));
        RevealIndexHypothesis {
            index: self.index,
            revealing_is_safe_hypothesis,
            need_testimony_hypothesis,
        }
        .into()
    }
}

impl Hypothesis for RevealIndexHypothesis {
    fn describe(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "Reveal {}", self.index)
    }

    fn evaluate(
        &mut self,
        _: &impl Log,
        _: Depth,
        _: &GameState,
        repository: impl HypothesisRepository,
    ) -> HypothesisEvaluation {
        let mut evaluator = repository.require_sub_evaluation(0.0);

        let reveal_result = evaluator.sub_evaluate(&self.revealing_is_safe_hypothesis);
        match &reveal_result {
            HypothesisResult::Pending(_) => {}
            HypothesisResult::Conclusive(fitness_and_action) => {
                if fitness_and_action.is_certain() {
                    return evaluator.finalize(HypothesisResult::Conclusive(
                        FitnessAndAction::certainty(Some(PlayerAction::TryReveal(
                            self.index.clone(),
                        ))),
                    ));
                }
            }
        }

        let info_desire_result = evaluator.sub_evaluate(&self.need_testimony_hypothesis);

        let fittest = or_result(reveal_result, info_desire_result);

        let result = fittest.map(|fitness| {
            FitnessAndAction::new(
                fitness.fitness(),
                Some(PlayerAction::TryReveal(self.index.clone())),
            )
        });

        evaluator.finalize(result)
    }
}
