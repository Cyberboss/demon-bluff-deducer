/// A repository of hypotheses available to a single `Hypothesis` during evaluation.
pub struct HypothesisRepository<'a, TLog>
where
    TLog: Log,
{
    inner: StackData<'a, TLog>,
}

impl<'a, TLog> HypothesisRepository<'a, TLog>
where
    TLog: Log,
{
    /// If a hypothesis has dependencies
    pub fn require_sub_evaluation(self, initial_fitness: f64) -> HypothesisEvaluator<'a, TLog> {
        let mut data = self.inner.current_data.borrow_mut();
        match &data.results[self.inner.current_reference().0] {
            Some(_) => {}
            None => {
                if let Some(previous) = self.inner.previous_data
                    && let Some(_) = &previous.results[self.inner.current_reference().0]
                {
                } else {
                    info!(logger: self.inner.log, "{} Set initial fitness: {}",self.inner.depth(), initial_fitness);
                }
                data.results[self.inner.current_reference().0] =
                    Some(HypothesisResult::Pending(FitnessAndAction {
                        action: HashSet::new(),
                        fitness: initial_fitness,
                    }));
            }
        }

        HypothesisEvaluator { inner: self.inner }
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
