use crate::engine::debugger::{desire_node::DesireNode, hypothesis_node::HypothesisNode};

pub enum Node {
    Hypothesis(HypothesisNode),
    Desire(DesireNode),
}
