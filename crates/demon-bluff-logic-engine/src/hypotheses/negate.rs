use demon_bluff_gameplay_engine::game_state::GameState;
use log::Log;

use crate::hypothesis::{
    Depth, Hypothesis, HypothesisReference, HypothesisRegistrar, HypothesisRepository,
    HypothesisResult, HypothesisReturn,
};

#[derive(Eq, PartialEq, Debug)]
pub struct NegateHypothesis {
    target_hypothesis: HypothesisReference,
}

impl NegateHypothesis {
    pub fn create<TLog>(
        _: &GameState,
        registrar: &mut HypothesisRegistrar<TLog>,
        target_hypothesis: HypothesisReference,
    ) -> HypothesisReference
    where
        TLog: Log,
    {
        registrar.register(Self { target_hypothesis })
    }
}

impl Hypothesis for NegateHypothesis {
    fn describe(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "Negate {}", self.target_hypothesis)
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

        let result = evaluator.sub_evaluate(&self.target_hypothesis);

        evaluator.create_return(match result {
            HypothesisResult::Pending(fitness_and_action) => {
                HypothesisResult::Pending(fitness_and_action.invert())
            }
            HypothesisResult::Conclusive(fitness_and_action) => {
                HypothesisResult::Conclusive(fitness_and_action.invert())
            }
        })
    }
}
