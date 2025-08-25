use log::Log;

use crate::engine::{desire::Desire, stack_data::StackData};

use super::{Hypothesis, HypothesisEvaluation, HypothesisResult};

pub trait HypothesisFinalizer {
    fn finalize(self, result: HypothesisResult) -> HypothesisEvaluation;
}

impl<'a, TLog, THypothesis, TDesire> HypothesisFinalizer
    for StackData<'a, TLog, THypothesis, TDesire>
where
    TLog: Log,
    THypothesis: Hypothesis,
    TDesire: Desire,
{
    fn finalize(self, result: HypothesisResult) -> HypothesisEvaluation {
        HypothesisEvaluation::new(result);
    }
}
