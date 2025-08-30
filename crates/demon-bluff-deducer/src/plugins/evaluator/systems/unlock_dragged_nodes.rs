use bevy::{
	ecs::{
		entity::Entity,
		query::With,
		system::{Commands, Query, Res, Single},
	},
	input::{ButtonInput, mouse::MouseButton},
	math::Vec2,
};

use crate::plugins::evaluator::components::{
	debugger_context::DebuggerContextComponent, node::NodeComponent,
	node_locked::NodeLockedComponent,
};

pub fn unlock_dragged_nodes(
	mut commands: Commands,
	buttons: Res<ButtonInput<MouseButton>>,
	mut context: Single<&mut DebuggerContextComponent>,
	locked_nodes: Query<(Entity, &NodeComponent), With<NodeLockedComponent>>,
) {
	if buttons.just_pressed(MouseButton::Middle) {
		for (entity, node) in locked_nodes {
			context.apply_node_delta(&node.node().node, Vec2::ZERO, false);
			commands.entity(entity).remove::<NodeLockedComponent>();
		}
	}
}
