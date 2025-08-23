use demon_bluff_gameplay_engine::{game_state::GameState, villager::VillagerIndex};
use log::Log;

use crate::{
    hypotheses::{
        is_evil::IsEvilHypothesis, need_testimony::NeedTestimonyHypothesis,
        revealing_is_safe::RevealingIsSafeHypothesis,
    },
    hypothesis::{
        Depth, FitnessAndAction, Hypothesis, HypothesisReference, HypothesisRegistrar,
        HypothesisRepository, HypothesisResult, HypothesisReturn, or_result,
    },
    player_action::PlayerAction,
};

#[derive(Eq, PartialEq, Debug)]
pub struct ExecuteIndexHypothesis {
    index: VillagerIndex,
    is_evil_hypothesis: HypothesisReference,
}

impl ExecuteIndexHypothesis {
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
        registrar.register(Self {
            index,
            is_evil_hypothesis,
        })
    }
}

impl Hypothesis for ExecuteIndexHypothesis {
    fn describe(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "Execute Villager {}", self.index)
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
        let result = match evaluator.sub_evaluate(&self.is_evil_hypothesis) {
            HypothesisResult::Pending(fitness_and_action) => {
                HypothesisResult::Pending(FitnessAndAction::new(
                    fitness_and_action.fitness(),
                    PlayerAction::TryExecute(self.index.clone()),
                ))
            }
            HypothesisResult::Conclusive(fitness_and_action) => {
                HypothesisResult::Conclusive(FitnessAndAction::new(
                    fitness_and_action.fitness(),
                    PlayerAction::TryExecute(self.index.clone()),
                ))
            }
        };
        evaluator.create_return(result)
    }
}
