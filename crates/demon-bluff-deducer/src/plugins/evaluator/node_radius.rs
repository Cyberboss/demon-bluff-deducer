use force_graph::NodeData;

use super::node::Node;

pub trait NodeRadius {
	fn radius(&self, is_root: bool) -> f32;
}

impl NodeRadius for NodeData<Node> {
	fn radius(&self, is_root: bool) -> f32 {
		5.0 * if is_root { 1.0 } else { self.mass }
	}
}
