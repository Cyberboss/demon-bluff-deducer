use log::{Log, info};

use crate::engine::{desire::Desire, hypothesis::HypothesisRepository, stack_data::StackData};

use super::HypothesisResult;

pub struct HypothesisInvocation<'a, TLog, TDesire>
where
    TLog: Log,
    TDesire: Desire,
{
    inner: StackData<'a, TLog, TDesire>,
}

impl<'a, TLog, TDesire> HypothesisInvocation<'a, TLog, TDesire>
where
    TLog: Log,
    TDesire: Desire,
{
    fn new(stack_data: StackData<'a, TLog, TDesire>) -> Self {
        Self { inner: stack_data }
    }

    fn enter(self) -> HypothesisResult {
        let reference = self.inner.current_reference();

        let mut hypothesis = self.inner.hypotheses[reference.0].borrow_mut();

        info!(logger: self.inner.log, "{} Entering: {}", self.inner.depth(), hypothesis);
        let repository = HypothesisRepository {
            inner: self.inner.share(),
        };

        let hypo_return = hypothesis.evaluate(
            self.inner.log,
            self.inner.depth(),
            self.inner.game_state,
            repository,
        );

        info!(logger: self.inner.log, "{} Result: {}", self.inner.depth(), hypo_return.result);

        if let HypothesisResult::Conclusive(_) = &hypo_return.result {
            for producer_reference in &self.inner.dependencies.desire_producers[reference.0] {
                let desire_data = self.inner.desire_data.borrow();
                if desire_data[producer_reference.0]
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

        let mut current_data = self.inner.current_data.borrow_mut();
        current_data.results[self.inner.current_reference().0] = Some(hypo_return.result.clone());

        hypo_return.result
    }
}
