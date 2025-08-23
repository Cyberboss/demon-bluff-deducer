use demon_bluff_gameplay_engine::{game_state::GameState, villager::VillagerIndex};
use log::Log;

use crate::{
    hypotheses::revealing_is_safe::RevealingIsSafeHypothesis,
    hypothesis::{
        Depth, FitnessAndAction, Hypothesis, HypothesisReference, HypothesisRegistrar,
        HypothesisRepository, HypothesisResult, HypothesisReturn,
    },
    player_action::PlayerAction,
};

#[derive(Eq, PartialEq, Debug)]
pub struct RevealIndexHypothesis {
    index: VillagerIndex,
    revealing_is_safe_hypothesis: HypothesisReference,
}

impl RevealIndexHypothesis {
    pub fn create<TLog>(
        game_state: &GameState,
        mut registrar: &mut HypothesisRegistrar<TLog>,
        index: VillagerIndex,
    ) -> HypothesisReference
    where
        TLog: Log,
    {
        let revealing_is_safe_hypothesis =
            RevealingIsSafeHypothesis::create(game_state, &mut registrar);
        registrar.register(Self {
            index,
            revealing_is_safe_hypothesis,
        })
    }
}

impl Hypothesis for RevealIndexHypothesis {
    fn describe(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "Reveal Villager #{}", self.index.0 + 1)
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

        match evaluator.sub_evaluate(&self.revealing_is_safe_hypothesis) {
            HypothesisResult::Pending(fitness_and_action) => todo!(),
            HypothesisResult::Conclusive(fitness_and_action) => {
                if fitness_and_action.is_certain() {
                    return evaluator.create_return(HypothesisResult::Conclusive(
                        FitnessAndAction::certainty(PlayerAction::TryReveal(self.index.clone())),
                    ));
                }
            }
        }

        evaluator.create_return(HypothesisResult::Conclusive(
            FitnessAndAction::unimplemented(),
        ))
    }
}
