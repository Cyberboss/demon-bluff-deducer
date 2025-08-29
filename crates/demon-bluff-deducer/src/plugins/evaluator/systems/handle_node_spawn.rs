use bevy::{
	asset::Assets,
	ecs::{
		event::EventReader,
		system::{Commands, ResMut},
	},
	math::primitives::Circle,
	render::mesh::{Mesh, Mesh2d},
	sprite::{ColorMaterial, MeshMaterial2d},
	transform::components::Transform,
};

use super::highlighting::{start_highlight_node, stop_highlight_node};
use crate::plugins::evaluator::{
	colours::COLOUR_NEUTRAL, components::node::NodeComponent, events::node_spawn::NodeSpawnEvent,
	node_radius::NodeRadius,
};

pub fn handle_node_spawn(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<ColorMaterial>>,
	mut event_reader: EventReader<NodeSpawnEvent>,
) {
	for event in event_reader.read() {
		let node = event.data();
		commands
			.spawn((
				NodeComponent::new(node.user_data.clone()),
				Mesh2d(meshes.add(Circle::new(node.radius(event.is_root())))),
				MeshMaterial2d(materials.add(COLOUR_NEUTRAL)),
				Transform::from_xyz(node.x, node.y, 0.0),
			))
			.observe(start_highlight_node)
			.observe(stop_highlight_node);
	}
}
