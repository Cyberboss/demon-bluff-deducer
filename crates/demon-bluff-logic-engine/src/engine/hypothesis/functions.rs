use std::fmt::Display;

use log::{Log, info};

use super::{Hypothesis, HypothesisEvaluation, HypothesisRepository, HypothesisResult};
use crate::{
    Breakpoint,
    engine::{
        DesireConsumerReference, DesireProducerReference, desire::Desire,
        fitness_and_action::FitnessAndAction, index_reference::IndexReference,
        stack_data::StackData,
    },
};

pub trait HypothesisFunctions {
    fn finalize(self, result: HypothesisResult) -> HypothesisEvaluation;
    fn set_desire(&mut self, desire_reference: &DesireProducerReference, desired: bool);
    fn desire_result(&self, desire_reference: &DesireConsumerReference) -> HypothesisResult;
}

impl<'a, TLog, THypothesis, TDesire, FDebugBreak> HypothesisFunctions
    for StackData<'a, TLog, THypothesis, TDesire, FDebugBreak>
where
    TLog: Log,
    THypothesis: Hypothesis,
    TDesire: Desire + Display,
    FDebugBreak: FnMut(Breakpoint) + Clone,
{
    fn finalize(self, result: HypothesisResult) -> HypothesisEvaluation {
        HypothesisEvaluation::new(result)
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

impl<'a, TLog, FDebugBreak> HypothesisFunctions for HypothesisRepository<'a, TLog, FDebugBreak>
where
    TLog: Log,
    FDebugBreak: FnMut(Breakpoint) + Clone,
{
    fn finalize(self, result: HypothesisResult) -> HypothesisEvaluation {
        self.stack_data.finalize(result)
    }

    fn set_desire(&mut self, desire_reference: &DesireProducerReference, desired: bool) {
        self.stack_data.set_desire(desire_reference, desired)
    }

    fn desire_result(&self, desire_reference: &DesireConsumerReference) -> HypothesisResult {
        self.stack_data.desire_result(desire_reference)
    }
}
