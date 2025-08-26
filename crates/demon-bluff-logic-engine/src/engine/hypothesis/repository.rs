use std::fmt::Display;

use log::{Log, info};

use crate::engine::{
    desire::Desire, fitness_and_action::FitnessAndAction, index_reference::IndexReference,
    stack_data::StackData,
};

use super::{Hypothesis, HypothesisEvaluator, HypothesisResult, functions::HypothesisFunctions};

/// A repository of hypotheses available to a single `Hypothesis` during evaluation.
pub trait HypothesisRepository: HypothesisFunctions {
    fn require_sub_evaluation(self, initial_fitness: f64) -> impl HypothesisEvaluator;
}

impl<'a, TLog, THypothesis, TDesire> HypothesisRepository
    for StackData<'a, TLog, THypothesis, TDesire>
where
    TLog: Log,
    THypothesis: Hypothesis + Display,
    TDesire: Desire + Display,
{
    /// If a hypothesis has dependencies
    fn require_sub_evaluation(self, initial_fitness: f64) -> impl HypothesisEvaluator {
        let mut data = self.current_data.borrow_mut();
        match &data.results[self.current_reference().index()] {
            Some(_) => {}
            None => {
                if let Some(previous) = self.previous_data
                    && let Some(_) = &previous.results[self.current_reference().index()]
                {
                } else {
                    info!(logger: self.log, "{} Set initial fitness: {}",self.depth(), initial_fitness);
                }
                data.results[self.current_reference().index()] = Some(HypothesisResult::Pending(
                    FitnessAndAction::new(initial_fitness, None),
                ));
            }
        }

        self
    }
}
