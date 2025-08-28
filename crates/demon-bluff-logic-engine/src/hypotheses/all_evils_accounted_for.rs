use std::collections::HashMap;

use demon_bluff_gameplay_engine::{game_state::GameState, villager::VillagerIndex};
use itertools::Itertools;
use log::Log;

use super::{DesireType, HypothesisBuilderType, is_evil::IsEvilHypothesisBuilder};
use crate::{
    engine::{
        Depth, FITNESS_UNKNOWN, FitnessAndAction, Hypothesis, HypothesisBuilder,
        HypothesisEvaluation, HypothesisEvaluator, HypothesisFunctions, HypothesisReference,
        HypothesisRegistrar, HypothesisRepository, HypothesisResult, and_result, or_result,
    },
    hypotheses::HypothesisType,
};

#[derive(Eq, PartialEq, Debug, Default, Clone)]
pub struct AllEvilsAccountedForHypothesisBuilder {}

impl HypothesisBuilder for AllEvilsAccountedForHypothesisBuilder {
    fn build(
        self,
        game_state: &GameState,
        registrar: &mut impl HypothesisRegistrar<HypothesisBuilderType, DesireType>,
    ) -> HypothesisType {
        let mut index_evil_hypotheses = HashMap::with_capacity(game_state.total_villagers());
        for index in game_state.villager_indicies() {
            index_evil_hypotheses.insert(
                index.clone(),
                registrar.register(IsEvilHypothesisBuilder::new(index)),
            );
        }

        AllEvilsAccountedForHypothesis {
            index_evil_hypotheses,
        }
        .into()
    }
}

/// Returns either 1 conclusive if certain where all evils are 0 pending otherwise
#[derive(Debug)]
pub struct AllEvilsAccountedForHypothesis {
    index_evil_hypotheses: HashMap<VillagerIndex, HypothesisReference>,
}

impl Hypothesis for AllEvilsAccountedForHypothesis {
    fn describe(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "All evils accounted for")
    }

    fn evaluate<TLog>(
        &mut self,
        _: &TLog,
        _: Depth,
        game_state: &GameState,
        repository: HypothesisRepository<TLog>,
    ) -> HypothesisEvaluation
    where
        TLog: Log,
    {
        if self.index_evil_hypotheses.is_empty() {
            return repository.finalize(HypothesisResult::Conclusive(FitnessAndAction::certainty(
                None,
            )));
        }

        let mut evaluator = repository.require_sub_evaluation(FITNESS_UNKNOWN);

        let mut villager_evil_probabilities = HashMap::with_capacity(game_state.total_villagers());
        for index in game_state.villager_indicies() {
            let sub_hypothesis = evaluator
                .sub_evaluate(self.index_evil_hypotheses.get(&index).expect(
                "You're telling me the villager indicies changed since building this hypothesis?",
            ));
            villager_evil_probabilities.insert(index, sub_hypothesis);
        }

        // for each combination of possible evils get the probability that they are the full set of evils
        // then or the results
        let mut individual_case_probabilities = Vec::new();
        for combo in game_state
            .villager_indicies()
            .combinations(game_state.total_evils() as usize)
        {
            let mut combo_probability = None;
            for index in combo.into_iter() {
                let index_evil_probability = villager_evil_probabilities.get(&index).expect("Bro, we literally calculated this two lines ago, is villager_indicies impure??");
                combo_probability = Some(match combo_probability {
                    Some(running_combo) => {
                        and_result(running_combo, index_evil_probability.clone())
                    }
                    None => index_evil_probability.clone(),
                })
            }

            individual_case_probabilities.push(
                combo_probability.expect("Getting tired of writing unreachable expect messages"),
            );
        }

        let mut final_probability = None;
        for case_probability in individual_case_probabilities.into_iter() {
            final_probability = Some(match final_probability {
                Some(running_combo) => or_result(running_combo, case_probability),
                None => case_probability,
            })
        }

        evaluator.finalize(final_probability.expect("One more for the road"))
    }
}
