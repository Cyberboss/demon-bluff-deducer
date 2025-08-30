use bevy::{
	ecs::{
		entity::Entity,
		observer::Trigger,
		query::With,
		system::{Commands, Query, Res, Single},
	},
	input::{ButtonInput, mouse::MouseButton},
	math::Vec2,
	picking::events::{Drag, Pointer},
	render::camera::{Camera, Projection},
};

use crate::plugins::evaluator::components::{
	debugger_context::DebuggerContextComponent, node::NodeComponent,
	node_locked::NodeLockedComponent,
};

pub fn drag_node(
	trigger: Trigger<Pointer<Drag>>,
	mut commands: Commands,
	mut context: Single<&mut DebuggerContextComponent>,
	nodes: Query<(Entity, &NodeComponent)>,
	projection: Single<&Projection, With<Camera>>,
) {
	let (entity, node) = nodes
		.get(trigger.target())
		.expect("This observer should not be attached to an entity without a NodeComponent!");

	let mut scale = 1.0;
	let projection = projection.into_inner();
	if let Projection::Orthographic(projection2d) = projection {
		scale = projection2d.scale;
	}

	context.apply_node_delta(&node.node().node, trigger.delta * scale, true);
	commands.entity(entity).insert(NodeLockedComponent);
}
