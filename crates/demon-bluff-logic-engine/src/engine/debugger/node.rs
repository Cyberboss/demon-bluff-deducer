use crate::engine::debugger::{desire_node::DesireNode, hypothesis_node::HypothesisNode};

#[derive(Debug)]
pub enum Node {
    Hypothesis(HypothesisNode),
    Desire(DesireNode),
}
