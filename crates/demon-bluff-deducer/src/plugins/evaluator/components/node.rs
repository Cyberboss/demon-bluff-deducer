use bevy::ecs::component::Component;

use crate::plugins::evaluator::node::Node;

#[derive(Component)]
pub struct NodeComponent {
	node: Node,
}

impl NodeComponent {
	pub fn new(node: Node) -> Self {
		Self { node }
	}

	pub fn node(&self) -> &Node {
		&self.node
	}
}
