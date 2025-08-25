use demon_bluff_gameplay_engine::{game_state::GameState, villager::VillagerIndex};
use log::Log;

use crate::{
    hypotheses::{HypothesisType, is_evil::IsEvilHypothesisBuilder},
    engine_old::{
        Depth, FitnessAndAction, Hypothesis, HypothesisBuilder, HypothesisReference,
        HypothesisRegistrar, HypothesisRepository, HypothesisResult, HypothesisReturn,
    },
    player_action::PlayerAction,
};

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct ExecuteIndexHypothesisBuilder {
    index: VillagerIndex,
}

#[derive(Debug)]
pub struct ExecuteIndexHypothesis {
    index: VillagerIndex,
    is_evil_hypothesis: HypothesisReference,
}

impl ExecuteIndexHypothesisBuilder {
    pub fn new(index: VillagerIndex) -> Self {
        Self { index }
    }
}

impl HypothesisBuilder for ExecuteIndexHypothesisBuilder {
    fn build<TLog>(self, _: &GameState, registrar: &mut HypothesisRegistrar<TLog>) -> HypothesisType
    where
        TLog: ::log::Log,
    {
        let is_evil_hypothesis =
            registrar.register(IsEvilHypothesisBuilder::new(self.index.clone()));
        ExecuteIndexHypothesis {
            is_evil_hypothesis,
            index: self.index,
        }
        .into()
    }
}

impl Hypothesis for ExecuteIndexHypothesis {
    fn describe(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "Execute Villager {}", self.index)
    }

    fn evaluate<TLog>(
        &mut self,
        _: &TLog,
        _: Depth,
        game_state: &GameState,
        repository: HypothesisRepository<TLog>,
    ) -> HypothesisReturn
    where
        TLog: Log,
    {
        let estimated_evils =
            (game_state.draw_stats().demons() + game_state.draw_stats().minions()) as f64;
        let total_villagers = game_state.draw_stats().total_villagers() as f64;

        let mut evaluator = repository.require_sub_evaluation(estimated_evils / total_villagers);
        let result = evaluator
            .sub_evaluate(&self.is_evil_hypothesis)
            .map(|fitness| {
                FitnessAndAction::new(
                    fitness.fitness(),
                    Some(PlayerAction::TryExecute(self.index.clone())),
                )
            });
        evaluator.create_return(result)
    }
}
