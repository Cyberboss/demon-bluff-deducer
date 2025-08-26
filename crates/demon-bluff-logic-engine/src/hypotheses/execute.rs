use demon_bluff_gameplay_engine::{game_state::GameState, villager::Villager};
use log::Log;

use crate::{
    engine::{
        Depth, Hypothesis, HypothesisBuilder, HypothesisEvaluation, HypothesisEvaluator,
        HypothesisFunctions, HypothesisReference, HypothesisRegistrar, HypothesisRepository,
        HypothesisResult, decide_result,
    },
    hypotheses::{HypothesisType, execute_index::ExecuteIndexHypothesisBuilder},
};

use super::{DesireType, HypothesisBuilderType};

#[derive(Eq, PartialEq, Debug, Clone, Default)]
pub struct ExecuteHypothesisBuilder {}

#[derive(Debug)]
pub struct ExecuteHypothesis {
    executable_hypotheses: Vec<HypothesisReference>,
}

impl HypothesisBuilder for ExecuteHypothesisBuilder {
    fn build(
        self,
        game_state: &GameState,
        registrar: &mut impl HypothesisRegistrar<HypothesisBuilderType, DesireType>,
    ) -> HypothesisType {
        let mut executable_hypotheses = Vec::new();
        game_state.iter_villagers(|villager_index, villager| match villager {
            Villager::Active(active_villager) => {
                if !active_villager.cant_kill() {
                    executable_hypotheses.push(
                        registrar.register(ExecuteIndexHypothesisBuilder::new(villager_index)),
                    );
                }
            }
            Villager::Hidden(hidden_villager) => {
                if !hidden_villager.cant_kill() {
                    executable_hypotheses.push(
                        registrar.register(ExecuteIndexHypothesisBuilder::new(villager_index)),
                    );
                }
            }
            Villager::Confirmed(_) => {}
        });

        ExecuteHypothesis {
            executable_hypotheses,
        }
        .into()
    }
}

impl Hypothesis for ExecuteHypothesis {
    fn describe(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "Execution Decision")
    }

    fn evaluate<TLog>(
        &mut self,
        _: &TLog,
        _: Depth,
        _: &GameState,
        repository: HypothesisRepository<TLog>,
    ) -> HypothesisEvaluation
    where
        TLog: Log,
    {
        if self.executable_hypotheses.is_empty() {
            return repository.finalize(HypothesisResult::impossible());
        }

        let mut evaluator = repository.require_sub_evaluation(0.0);
        let mut result = None;
        for reference in &self.executable_hypotheses {
            let sub_evaluation = evaluator.sub_evaluate(reference);
            result = Some(match result {
                Some(existing_fitness) => decide_result(sub_evaluation, existing_fitness),
                None => sub_evaluation,
            })
        }

        let result = result.expect("There should be a result after iterating");
        evaluator.finalize(result)
    }
}
