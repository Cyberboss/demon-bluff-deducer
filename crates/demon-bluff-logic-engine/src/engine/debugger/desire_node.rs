use crate::hypotheses::DesireType;

#[derive(Debug)]
pub struct DesireNode {
	desire_type: DesireType,
	pending: usize,
	desired: usize,
	undesired: usize,
}

impl DesireNode {
	fn new(desire_type: DesireType, producers: usize) -> Self {
		Self {
			desire_type,
			pending: producers,
			desired: 0,
			undesired: 0,
		}
	}
}

pub fn create_desire_node(desire_type: DesireType, producers: usize) -> DesireNode {
	DesireNode::new(desire_type, producers)
}

pub fn update_desire_node(node: &mut DesireNode, pending: usize, desired: usize, undesired: usize) {
	node.pending = pending;
	node.desired = desired;
	node.undesired = undesired;
}
