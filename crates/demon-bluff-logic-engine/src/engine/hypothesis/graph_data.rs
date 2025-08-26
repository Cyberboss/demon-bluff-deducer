use crate::engine::{
    dependencies::DependencyData,
    desire::{Desire, DesireDefinition},
};

use super::{Hypothesis, reference::HypothesisReference};

#[derive(Debug)]
pub struct HypothesisGraphData<THypotheses, TDesire>
where
    THypotheses: Hypothesis,
    TDesire: Desire,
{
    pub root: HypothesisReference,
    pub hypotheses: Vec<THypotheses>,
    pub dependencies: DependencyData,
    pub desires: Vec<DesireDefinition<TDesire>>,
}

impl<THypothesis, TDesire> HypothesisGraphData<THypothesis, TDesire>
where
    THypothesis: Hypothesis,
    TDesire: Desire,
{
    pub fn new(
        root: HypothesisReference,
        hypotheses: Vec<THypothesis>,
        dependencies: DependencyData,
        desires: Vec<DesireDefinition<TDesire>>,
    ) -> Self {
        Self {
            root,
            hypotheses,
            dependencies,
            desires,
        }
    }
}
