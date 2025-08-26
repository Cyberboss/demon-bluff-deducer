use std::fmt::Display;

use log::{Log, info};

use crate::engine::{desire::Desire, index_reference::IndexReference, stack_data::StackData};

use super::{Hypothesis, HypothesisResult};

pub trait HypothesisInvocation {
    fn invoke(&mut self) -> HypothesisResult;
}

impl<'a, TLog, THypothesis, TDesire> HypothesisInvocation
    for StackData<'a, TLog, THypothesis, TDesire>
where
    TLog: Log,
    THypothesis: Hypothesis + Display,
    TDesire: Desire + Display,
{
    fn invoke(&mut self) -> HypothesisResult {
        let reference = self.current_reference();

        let mut hypothesis = self.hypotheses[reference.index()].borrow_mut();

        info!(logger: self.log, "{} Entering: {}", self.depth(), hypothesis);

        let hypo_return =
            hypothesis.evaluate(self.log, self.depth(), self.game_state, self.share());

        let result = hypo_return.unpack();
        info!(logger: self.log, "{} Result: {}", self.depth(), result);

        if let HypothesisResult::Conclusive(_) = &result {
            for producer_reference in &self.dependencies.desire_producers[reference.index()] {
                let desire_data = self.desire_data.borrow();
                if desire_data[producer_reference.index()]
                    .pending
                    .iter()
                    .any(|pending_reference| pending_reference == reference)
                {
                    panic!(
                        "{}: {} was supposed to produce a result for {} before concluding but didn't!",
                        reference, hypothesis, producer_reference
                    )
                }
            }
        }

        let mut current_data = self.current_data.borrow_mut();
        current_data.results[self.current_reference().index()] = Some(result.clone());

        result
    }
}
