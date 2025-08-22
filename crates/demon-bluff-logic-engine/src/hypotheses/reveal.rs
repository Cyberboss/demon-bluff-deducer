use std::collections::HashMap;

use demon_bluff_gameplay_engine::{
    game_state::GameState,
    villager::{Villager, VillagerIndex},
};
use log::{Log, info};

use crate::hypothesis::{
    Depth, FitnessAndAction, Hypothesis, HypothesisReference, HypothesisRegistrar,
    HypothesisRepository, HypothesisResult, HypothesisReturn, fittest_result,
};

use super::reveal_index::RevealIndexHypothesis;

#[derive(Eq, PartialEq, Debug)]
pub struct RevealHypothesis {
    revealable_indexes: HashMap<VillagerIndex, HypothesisReference>,
}

impl RevealHypothesis {
    pub fn create(
        game_state: &GameState,
        registrar: &mut HypothesisRegistrar,
    ) -> HypothesisReference {
        let mut revealable_indexes = HashMap::new();
        game_state.iter_villagers(|villager_index, villager| match villager {
            Villager::Active(_) | Villager::Confirmed(_) => {}
            Villager::Hidden(hidden_villager) => {
                if !hidden_villager.cant_reveal() {
                    revealable_indexes.insert(
                        villager_index.clone(),
                        RevealIndexHypothesis::create(game_state, registrar, villager_index),
                    );
                }
            }
        });

        registrar.register(Self { revealable_indexes })
    }
}

impl Hypothesis for RevealHypothesis {
    fn describe(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "Reveal a Villager")
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
        if self.revealable_indexes.is_empty() {
            return repository
                .create_return(HypothesisResult::Conclusive(FitnessAndAction::impossible()));
        }

        let mut evaluator = repository.require_sub_evaluation(0.0);
        let mut result = None;
        for (index, reference) in self.revealable_indexes.iter() {
            let sub_evaluation = evaluator.sub_evaluate(reference);
            result = Some(match result {
                Some(existing_fitness) => fittest_result(sub_evaluation, existing_fitness),
                None => sub_evaluation,
            })
        }

        let result = result.expect("There should be a result after iterating");
        evaluator.create_return(result)
    }
}
