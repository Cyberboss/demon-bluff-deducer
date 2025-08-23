use std::collections::HashMap;

use demon_bluff_gameplay_engine::{
    game_state::GameState,
    villager::{Villager, VillagerIndex},
};
use log::Log;

use crate::{
    hypotheses::execute_index::ExecuteIndexHypothesis,
    hypothesis::{
        Depth, FitnessAndAction, Hypothesis, HypothesisReference, HypothesisRegistrar,
        HypothesisRepository, HypothesisResult, HypothesisReturn, or_result,
    },
};

#[derive(Eq, PartialEq, Debug)]
pub struct ExecuteHypothesis {
    executable_hypotheses: Vec<HypothesisReference>,
}

impl ExecuteHypothesis {
    pub fn create<TLog>(
        game_state: &GameState,
        registrar: &mut HypothesisRegistrar<TLog>,
    ) -> HypothesisReference
    where
        TLog: Log,
    {
        let mut executable_hypotheses = Vec::new();
        game_state.iter_villagers(|villager_index, villager| match villager {
            Villager::Active(active_villager) => {
                if !active_villager.cant_kill() {
                    executable_hypotheses.push(ExecuteIndexHypothesis::create(
                        game_state,
                        registrar,
                        villager_index,
                    ));
                }
            }
            Villager::Hidden(hidden_villager) => {
                if !hidden_villager.cant_kill() {
                    executable_hypotheses.push(ExecuteIndexHypothesis::create(
                        game_state,
                        registrar,
                        villager_index,
                    ));
                }
            }
            Villager::Confirmed(_) => {}
        });

        registrar.register(Self {
            executable_hypotheses,
        })
    }
}

impl Hypothesis for ExecuteHypothesis {
    fn describe(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "Execution Decision")
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
        if self.executable_hypotheses.is_empty() {
            return repository
                .create_return(HypothesisResult::Conclusive(FitnessAndAction::impossible()));
        }

        let mut evaluator = repository.require_sub_evaluation(0.0);
        let mut result = None;
        for reference in &self.executable_hypotheses {
            let sub_evaluation = evaluator.sub_evaluate(reference);
            result = Some(match result {
                Some(existing_fitness) => or_result(sub_evaluation, existing_fitness),
                None => sub_evaluation,
            })
        }

        let result = result.expect("There should be a result after iterating");
        evaluator.create_return(result)
    }
}
