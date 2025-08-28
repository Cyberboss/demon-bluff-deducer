use std::{
    collections::HashMap,
    fmt::{Debug, Formatter},
};

use force_graph::{DefaultNodeIdx, ForceGraph};

use super::HypothesisReference;

pub(super) struct GraphBuilder {
    graph: ForceGraph<GraphNodeData>,
    node_map: HashMap<HypothesisReference, DefaultNodeIdx>,
}

#[derive(Debug)]
pub struct GraphNodeData {
    description: String,
    current_fitness: Option<f64>,
}

impl Debug for GraphBuilder {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GraphBuilder")
            //.field("graph", &self.graph)
            .field("node_map", &self.node_map)
            .finish()
    }
}
