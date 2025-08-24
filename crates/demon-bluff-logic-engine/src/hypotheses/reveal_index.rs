use demon_bluff_gameplay_engine::{game_state::GameState, villager::VillagerIndex};
use log::Log;

use crate::{
    hypotheses::{
        HypothesisType,
        need_testimony::{NeedTestimonyHypothesis, NeedTestimonyHypothesisBuilder},
        revealing_is_safe::{RevealingIsSafeHypothesis, RevealingIsSafeHypothesisBuilder},
    },
    hypothesis::{
        Depth, FitnessAndAction, Hypothesis, HypothesisBuilder, HypothesisReference,
        HypothesisRegistrar, HypothesisRepository, HypothesisResult, HypothesisReturn, or_result,
    },
    player_action::PlayerAction,
};

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
    type HypothesisImpl = RevealIndexHypothesis;

    fn build<TLog>(
        self,
        game_state: &::demon_bluff_gameplay_engine::game_state::GameState,
        registrar: &mut crate::hypothesis::HypothesisRegistrar<TLog>,
    ) -> Self::HypothesisImpl
    where
        Self::HypothesisImpl: Hypothesis,
        HypothesisType: From<Self::HypothesisImpl>,
        TLog: ::log::Log,
    {
        let revealing_is_safe_hypothesis =
            registrar.register(RevealingIsSafeHypothesisBuilder::default());
        let need_testimony_hypothesis =
            registrar.register(NeedTestimonyHypothesisBuilder::new(self.index.clone()));
        Self::HypothesisImpl {
            index: self.index,
            revealing_is_safe_hypothesis,
            need_testimony_hypothesis,
        }
    }
}

impl Hypothesis for RevealIndexHypothesis {
    fn describe(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "Reveal {}", self.index)
    }

    fn evaluate<TLog>(
        &mut self,
        log: &TLog,
        depth: Depth,
        game_state: &GameState,
        repository: HypothesisRepository<TLog>,
    ) -> HypothesisReturn
    where
        TLog: Log,
    {
        let mut evaluator = repository.require_sub_evaluation(0.0);

        let reveal_result = evaluator.sub_evaluate(&self.revealing_is_safe_hypothesis);
        match &reveal_result {
            HypothesisResult::Pending(_) => {}
            HypothesisResult::Conclusive(fitness_and_action) => {
                if fitness_and_action.is_certain() {
                    return evaluator.create_return(HypothesisResult::Conclusive(
                        FitnessAndAction::certainty(Some(PlayerAction::TryReveal(
                            self.index.clone(),
                        ))),
                    ));
                }
            }
        }

        let info_desire_result = evaluator.sub_evaluate(&self.need_testimony_hypothesis);

        let fittest = or_result(reveal_result, info_desire_result);

        evaluator.create_return(match fittest {
            HypothesisResult::Pending(fitness_and_action) => {
                HypothesisResult::Pending(FitnessAndAction::new(
                    fitness_and_action.fitness(),
                    Some(PlayerAction::TryReveal(self.index.clone())),
                ))
            }
            HypothesisResult::Conclusive(fitness_and_action) => {
                HypothesisResult::Conclusive(FitnessAndAction::new(
                    fitness_and_action.fitness(),
                    Some(PlayerAction::TryReveal(self.index.clone())),
                ))
            }
        })
    }
}
