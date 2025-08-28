use log::{Log, info};

use super::{Hypothesis, HypothesisResult};
use crate::{
    Breakpoint,
    engine::{HypothesisRepository, index_reference::IndexReference, stack_data::StackData},
    hypotheses::{DesireType, HypothesisType},
};

pub trait HypothesisInvocation {
    fn invoke(&mut self) -> HypothesisResult;
}

impl<'a, TLog, FDebugBreak> HypothesisInvocation
    for StackData<'a, TLog, HypothesisType, DesireType, FDebugBreak>
where
    TLog: Log,
    FDebugBreak: FnMut(Breakpoint) + Clone,
{
    fn invoke(&mut self) -> HypothesisResult {
        let reference = self.current_reference();

        let mut hypothesis = self.hypotheses[reference.index()].borrow_mut();

        info!(logger: self.log, "{} Entering: {}", self.depth(), hypothesis);

        let hypo_return = hypothesis.evaluate(
            self.log,
            self.depth(),
            self.game_state,
            HypothesisRepository::new(self.share()),
        );

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
                        "{reference}: {hypothesis} was supposed to produce a result for {producer_reference} before concluding but didn't!"
                    )
                }
            }
        }

        let mut current_data = self.current_data.borrow_mut();
        current_data.results[self.current_reference().index()] = Some(result.clone());

        result
    }
}
