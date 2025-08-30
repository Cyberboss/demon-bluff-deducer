use bevy::math::Vec2;

use super::node::Node;

#[derive(Debug, Clone)]
pub struct NodeAndLocked {
	pub node: Node,
	pub locked_coordinate: Option<Vec2>,
}
