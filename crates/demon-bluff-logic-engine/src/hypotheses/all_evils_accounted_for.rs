use demon_bluff_gameplay_engine::game_state::GameState;
use log::Log;

use crate::{
    engine::{
        Depth, FitnessAndAction, Hypothesis, HypothesisBuilder, HypothesisEvaluation,
        HypothesisEvaluator, HypothesisFunctions, HypothesisReference, HypothesisRegistrar,
        HypothesisRepository, HypothesisResult,
    },
    hypotheses::HypothesisType,
};

use super::{DesireType, HypothesisBuilderType, is_evil::IsEvilHypothesisBuilder};

#[derive(Eq, PartialEq, Debug, Default, Clone)]
pub struct AllEvilsAccountedForHypothesisBuilder {}

impl HypothesisBuilder for AllEvilsAccountedForHypothesisBuilder {
    fn build(
        self,
        game_state: &GameState,
        registrar: &mut impl HypothesisRegistrar<HypothesisBuilderType, DesireType>,
    ) -> HypothesisType {
        let mut index_evil_hypotheses = Vec::new();
        game_state.iter_villagers(|index, _| {
            index_evil_hypotheses.push(registrar.register(IsEvilHypothesisBuilder::new(index)));
        });

        AllEvilsAccountedForHypothesis {
            index_evil_hypotheses,
        }
        .into()
    }
}

/// Returns either 1 conclusive if certain where all evils are 0 pending otherwise
#[derive(Debug)]
pub struct AllEvilsAccountedForHypothesis {
    index_evil_hypotheses: Vec<HypothesisReference>,
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

        let mut evaluator = repository.require_sub_evaluation(0.0);
        let mut evils_found = 0;
        for reference in &self.index_evil_hypotheses {
            let sub_evaluation = evaluator.sub_evaluate(reference);
            match sub_evaluation {
                HypothesisResult::Pending(_) => {
                    return evaluator
                        .finalize(HypothesisResult::Pending(FitnessAndAction::impossible()));
                }
                HypothesisResult::Conclusive(fitness_and_action) => {
                    if fitness_and_action.is_certain() {
                        evils_found = evils_found + 1;
                    }
                }
            }
        }

        evaluator.finalize(if evils_found >= game_state.total_evils() {
            HypothesisResult::Conclusive(FitnessAndAction::certainty(None))
        } else {
            HypothesisResult::Pending(FitnessAndAction::impossible())
        })
    }
}
