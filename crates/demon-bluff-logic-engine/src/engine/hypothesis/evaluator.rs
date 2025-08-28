use log::{Log, info};

use super::{HypothesisFunctions, reference::HypothesisReference, result::HypothesisResult};
use crate::{
    engine::{
        DesireConsumerReference, DesireProducerReference, fitness_and_action::FitnessAndAction,
        hypothesis::invocation::HypothesisInvocation, index_reference::IndexReference,
        stack_data::StackData,
    },
    hypotheses::{DesireType, HypothesisType},
};

/// Used to evaluate sub-hypotheses via their `HypothesisReference`s.
pub trait HypothesisEvaluator<'a, TLog, THypothesis, TDesire>: HypothesisFunctions {
    fn sub_evaluate(&mut self, hypothesis_reference: &HypothesisReference) -> HypothesisResult;
    fn set_desire(&mut self, desire_reference: &DesireProducerReference, desired: bool);
    fn desire_result(&self, desire_reference: &DesireConsumerReference) -> HypothesisResult;
}

impl<'a, TLog> HypothesisEvaluator<'a, TLog, HypothesisType, DesireType>
    for StackData<'a, TLog, HypothesisType, DesireType>
where
    TLog: Log,
{
    fn sub_evaluate(&mut self, hypothesis_reference: &HypothesisReference) -> HypothesisResult {
        let current_reference = self.current_reference();

        let mut current_data = self.current_data.borrow_mut();
        let mut force_conclusive = false;
        if let Some(break_at) = self.break_at
            && break_at == current_reference
        {
            info!(
                logger: self.log,
                "{} Want to evaluate {} but we are breaking the cycle",
                self.depth(),
                hypothesis_reference
            );

            force_conclusive = true;
        } else if let Some(previous_data) = self.previous_data
            && let Some(HypothesisResult::Conclusive(previously_conclusive_result)) =
                &previous_data.results[hypothesis_reference.index()]
        {
            info!(logger: self.log, "{} Skipping previously concluded hypothesis: {}", self.depth(), hypothesis_reference);
            current_data.results[hypothesis_reference.index()] = Some(
                HypothesisResult::Conclusive(previously_conclusive_result.clone()),
            );
        } else {
            match self.hypotheses[hypothesis_reference.index()].try_borrow_mut() {
                Ok(next_reference) => {
                    // Important or entering the invocation will BorrowError
                    drop(current_data);
                    drop(next_reference);

                    let mut invocation = self.push(hypothesis_reference.clone());

                    return invocation.invoke();
                }
                Err(_) => {
                    info!(
                        logger: self.log,
                        "{} Cycle detected when trying to evaluate reference {}",
                        self.depth(),
                        hypothesis_reference
                    );

                    let cycle = self.create_cycle(hypothesis_reference);

                    let mut cycles = self.cycles.borrow_mut();
                    cycles.insert(cycle);
                }
            }
        }

        let relevant_iteration_data = current_data.results[hypothesis_reference.index()]
            .as_ref()
            .unwrap_or_else(|| {
                self.previous_data
                    .expect("We shouldn't be using cached fitness data if none exists")
                    .results[hypothesis_reference.index()]
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
            logger: self.log,
            "{} Using existing {} result: {}",
            self.depth(),
            hypothesis_reference,
            last_evaluate
        );

        last_evaluate
    }

    fn set_desire(&mut self, desire_reference: &DesireProducerReference, desired: bool) {
        let mut borrow = self.desire_data.borrow_mut();
        let data = &mut borrow[desire_reference.index()];

        let current_reference = self.current_reference();
        data.pending.remove(current_reference);

        if desired {
            data.desired.insert(current_reference.clone());
            data.undesired.remove(current_reference);
        } else {
            data.undesired.insert(current_reference.clone());
            data.desired.remove(current_reference);
        }

        info!(logger: self.log, "{} Set {}: {}. Now {}", self.depth(), desire_reference, desired, data);
    }

    fn desire_result(&self, desire_reference: &DesireConsumerReference) -> HypothesisResult {
        let definition = &self.desire_definitions[desire_reference.index()];
        let borrow = self.desire_data.borrow();

        let data = &borrow[desire_reference.index()];

        info!(logger: self.log, "{} Read desire {} {} - {}", self.depth(), desire_reference, definition.desire(), data);
        let total = data.total();
        let fitness = FitnessAndAction::new(
            if data.desired.is_empty() {
                0.0 // stop divide by 0
            } else {
                (data.desired.len() as f64) / (total as f64)
            },
            None,
        );
        if data.pending.is_empty() {
            HypothesisResult::Conclusive(fitness)
        } else {
            info!(logger: self.log, "{} Remaining Pending: {}", self.depth(), data.pending.iter().map(|producer_hypothesis_reference| format!("{producer_hypothesis_reference}")).collect::<Vec<String>>().join(", "));
            HypothesisResult::Pending(fitness)
        }
    }
}
