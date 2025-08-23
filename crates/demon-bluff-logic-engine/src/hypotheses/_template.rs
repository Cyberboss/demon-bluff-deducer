use demon_bluff_gameplay_engine::game_state::GameState;
use log::Log;

use crate::hypothesis::{
    Depth, FITNESS_UNKNOWN, Hypothesis, HypothesisReference, HypothesisRegistrar,
    HypothesisRepository, HypothesisReturn,
};

#[derive(Eq, PartialEq, Debug)]
pub struct TemplateHypothesis {
    sub_hypothesis: HypothesisReference,
}

impl TemplateHypothesis {
    pub fn create<TLog>(
        _: &GameState,
        mut registrar: &mut HypothesisRegistrar<TLog>,
    ) -> HypothesisReference
    where
        TLog: Log,
    {
        let sub_hypothesis = TemplateHypothesis::create(game_state, &mut registrar);
        registrar.register(Self { sub_hypothesis })
    }
}

impl Hypothesis for TemplateHypothesis {
    fn describe(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "REPLACE ME")
    }

    fn evaluate<TLog>(
        &mut self,
        _: &TLog,
        _: Depth,
        _: &GameState,
        repository: HypothesisRepository<TLog>,
    ) -> HypothesisReturn
    where
        TLog: Log,
    {
        let mut evaluator = repository.require_sub_evaluation(FITNESS_UNKNOWN);

        let sub_result = evaluator.sub_evaluate(&self.sub_hypothesis);

        evaluator.create_return(sub_result)
    }
}
