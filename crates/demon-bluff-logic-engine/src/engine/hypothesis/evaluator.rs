use log::{Log, info};

use crate::engine::stack_data::StackData;

use super::{reference::HypothesisReference, result::HypothesisResult};

/// Used to evaluate sub-hypotheses via their `HypothesisReference`s.
#[derive(Debug)]
pub struct HypothesisEvaluator<'a, TLog>
where
    TLog: Log,
{
    inner: StackData<'a, TLog>,
}

impl<'a, TLog> HypothesisEvaluator<'a, TLog>
where
    TLog: Log,
{
    pub fn sub_evaluate(&mut self, hypothesis_reference: &HypothesisReference) -> HypothesisResult {
        let current_reference = self
            .inner
            .current_reference()
            .expect("There should be at least one reference in the stack");

        let mut current_data = self.inner.current_data.borrow_mut();
        let mut force_conclusive = false;
        if let Some(break_at) = self.inner.break_at
            && break_at == current_reference
        {
            info!(
                logger: self.inner.log,
                "{} Want to evaluate {} but we are breaking the cycle",
                self.inner.depth(),
                hypothesis_reference
            );

            force_conclusive = true;
        } else {
            if let Some(previous_data) = self.inner.previous_data
                && let Some(HypothesisResult::Conclusive(previously_conclusive_result)) =
                    &previous_data.results[hypothesis_reference.0]
            {
                info!(logger: self.inner.log, "{} Skipping previously concluded hypothesis: {}", self.inner.depth(), hypothesis_reference);
                current_data.results[hypothesis_reference.0] = Some(HypothesisResult::Conclusive(
                    previously_conclusive_result.clone(),
                ));
            } else {
                match self.inner.hypotheses[hypothesis_reference.0].try_borrow_mut() {
                    Ok(next_reference) => {
                        // Important or entering the invocation will BorrowError
                        drop(current_data);
                        drop(next_reference);

                        let invocation = HypothesisInvocation {
                            inner: self.inner.push(hypothesis_reference.clone()),
                        };

                        return invocation.enter();
                    }
                    Err(_) => {
                        info!(
                            logger: self.inner.log,
                            "{} Cycle detected when trying to evaluate reference {}",
                            self.inner.depth(),
                            hypothesis_reference
                        );

                        let cycle = self.inner.into_cycle(hypothesis_reference);

                        let mut cycles = self.inner.cycles.borrow_mut();
                        cycles.insert(cycle);
                    }
                }
            }
        }

        let relevant_iteration_data = current_data.results[hypothesis_reference.0]
            .as_ref()
            .unwrap_or_else(|| {
                self.inner
                    .previous_data
                    .expect("We shouldn't be using cached fitness data if none exists")
                    .results[hypothesis_reference.0]
                    .as_ref()
                    .expect("Fitness for cycle break didn't previously exist")
            });

        let mut last_evaluate = relevant_iteration_data.clone();

        if force_conclusive {
            last_evaluate = HypothesisResult::Conclusive(match last_evaluate {
                HypothesisResult::Pending(fitness_and_action)
                | HypothesisResult::Conclusive(fitness_and_action) => fitness_and_action,
            })
        }

        info!(
            logger: self.inner.log,
            "{} Using existing {} result: {}",
            self.inner.depth(),
            hypothesis_reference,
            last_evaluate
        );

        last_evaluate
    }

    pub fn set_desire(&mut self, desire_reference: &DesireProducerReference, desired: bool) {
        let mut borrow = self.inner.desire_data.borrow_mut();
        let data = &mut borrow[desire_reference.0];

        let current_reference = self.inner.current_reference();
        data.pending.remove(current_reference);

        if desired {
            data.desired.insert(current_reference.clone());
            data.undesired.remove(current_reference);
        } else {
            data.undesired.insert(current_reference.clone());
            data.desired.remove(current_reference);
        }

        info!(logger: self.inner.log, "{} Set {}: {}. Now {}", self.inner.depth(), desire_reference, desired, data);
    }

    pub fn desire_result(&self, desire_reference: &DesireConsumerReference) -> HypothesisResult {
        let definition = &self.inner.desire_definitions[desire_reference.0];
        let borrow = self.inner.desire_data.borrow();

        let data = &borrow[desire_reference.0];

        info!(logger: self.inner.log, "{} Read desire {} {} - {}", self.inner.depth(), desire_reference, definition.desire, data);
        let total = data.total();
        let fitness = FitnessAndAction::new(
            if data.desired.len() == 0 {
                0.0 // stop divide by 0
            } else {
                (data.desired.len() as f64) / (total as f64)
            },
            None,
        );
        if data.pending.len() == 0 {
            HypothesisResult::Conclusive(fitness)
        } else {
            info!(logger: self.inner.log, "{} Remaining Pending: {}", self.inner.depth(), data.pending.iter().map(|producer_hypothesis_reference| format!("{}", producer_hypothesis_reference)).collect::<Vec<String>>().join(", "));
            HypothesisResult::Pending(fitness)
        }
    }

    pub fn create_return(self, result: HypothesisResult) -> HypothesisReturn {
        HypothesisReturn { result }
    }
}
