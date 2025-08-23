use std::collections::HashMap;

use demon_bluff_gameplay_engine::{
    game_state::GameState,
    villager::{Villager, VillagerIndex},
};
use log::{Log, info};

use crate::hypothesis::{
    Depth, FitnessAndAction, Hypothesis, HypothesisReference, HypothesisRegistrar,
    HypothesisRepository, HypothesisResult, HypothesisReturn, or_result,
};

use super::reveal_index::RevealIndexHypothesis;

#[derive(Eq, PartialEq, Debug)]
pub struct RevealHypothesis {
    revealable_hypotheses: Vec<HypothesisReference>,
}

impl RevealHypothesis {
    pub fn create<TLog>(
        game_state: &GameState,
        registrar: &mut HypothesisRegistrar<TLog>,
    ) -> HypothesisReference
    where
        TLog: Log,
    {
        let mut revealable_hypotheses = Vec::new();
        game_state.iter_villagers(|villager_index, villager| match villager {
            Villager::Active(_) | Villager::Confirmed(_) => {}
            Villager::Hidden(hidden_villager) => {
                if !hidden_villager.cant_reveal() {
                    revealable_hypotheses.push(RevealIndexHypothesis::create(
                        game_state,
                        registrar,
                        villager_index,
                    ));
                }
            }
        });

        registrar.register(Self {
            revealable_hypotheses,
        })
    }
}

impl Hypothesis for RevealHypothesis {
    fn describe(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "Reveal Decision")
    }

    fn evaluate<'a, 'b, TLog>(
        &'a mut self,
        log: &'a TLog,
        depth: Depth,
        game_state: &'a GameState,
        repository: HypothesisRepository<'b, TLog>,
    ) -> HypothesisReturn
    where
        TLog: Log,
    {
        if self.revealable_hypotheses.is_empty() {
            return repository
                .create_return(HypothesisResult::Conclusive(FitnessAndAction::impossible()));
        }

        let mut evaluator = repository.require_sub_evaluation(0.0);
        let mut result = None;
        for reference in &self.revealable_hypotheses {
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
