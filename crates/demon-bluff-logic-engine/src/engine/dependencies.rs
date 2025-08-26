use super::{DesireConsumerReference, DesireProducerReference, HypothesisReference};

#[derive(Debug, Default)]
pub struct DependencyData {
    pub desire_producers: Vec<Vec<DesireProducerReference>>,
    pub desire_consumers: Vec<Vec<DesireConsumerReference>>,
    pub hypotheses: Vec<Vec<HypothesisReference>>,
}
