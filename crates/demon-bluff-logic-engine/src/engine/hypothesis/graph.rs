use crate::{
    engine::{dependencies::DependencyData, desire::DesireDefinition},
    hypotheses::HypothesisType,
};

use super::reference::HypothesisReference;

#[derive(Debug)]
pub struct HypothesisGraph {
    root: HypothesisReference,
    hypotheses: Vec<HypothesisType>,
    dependencies: DependencyData,
    desires: Vec<DesireDefinition>,
}
