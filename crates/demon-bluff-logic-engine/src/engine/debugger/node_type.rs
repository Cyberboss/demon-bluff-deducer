use crate::engine::debugger::{desire_node::DesireNode, hypothesis_node::HypothesisNode};

pub enum NodeType {
    Hypothesis(HypothesisNode),
    Desire(DesireNode),
}
