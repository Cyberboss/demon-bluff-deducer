use bevy::ecs::event::Event;
use force_graph::NodeData;

use crate::plugins::evaluator::node::Node;

#[derive(Event)]
pub struct NodeSpawnEvent {
	node_data: NodeData<Node>,
	is_root: bool,
}

impl NodeSpawnEvent {
	pub fn new(node_data: NodeData<Node>, is_root: bool) -> Self {
		Self { node_data, is_root }
	}

	pub fn data(&self) -> &NodeData<Node> {
		&self.node_data
	}

	pub fn is_root(&self) -> bool {
		self.is_root
	}
}
