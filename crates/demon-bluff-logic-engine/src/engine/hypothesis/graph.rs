use crate::engine::{
    dependencies::DependencyData,
    desire::{Desire, DesireDefinition},
};

use super::{Hypothesis, reference::HypothesisReference};

#[derive(Debug)]
pub struct HypothesisGraph<THypotheses, TDesire>
where
    THypotheses: Hypothesis,
    TDesire: Desire,
{
    root: HypothesisReference,
    hypotheses: Vec<THypotheses>,
    dependencies: DependencyData,
    desires: Vec<DesireDefinition<TDesire>>,
}

impl<THypothesis, TDesire> HypothesisGraph<THypothesis, TDesire>
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
