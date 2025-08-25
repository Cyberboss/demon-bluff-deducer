use super::{DesireConsumerReference, DesireProducerReference, HypothesisReference};

#[derive(Debug, Default)]
pub struct DependencyData {
    desire_producers: Vec<Vec<DesireProducerReference>>,
    desire_consumers: Vec<Vec<DesireConsumerReference>>,
    hypotheses: Vec<Vec<HypothesisReference>>,
}
