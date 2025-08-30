use bevy::ecs::component::Component;

use crate::plugins::evaluator::node_data::NodeAndLocked;

#[derive(Component)]
pub struct NodeComponent {
	node: NodeAndLocked,
}

impl NodeComponent {
	pub fn new(node: NodeAndLocked) -> Self {
		Self { node }
	}

	pub fn node(&self) -> &NodeAndLocked {
		&self.node
	}
}
