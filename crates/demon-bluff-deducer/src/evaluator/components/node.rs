use demon_bluff_logic_engine::{DesireNode, HypothesisNode};

pub enum Node {
	Hypothesis(usize),
	Desire(usize),
}
