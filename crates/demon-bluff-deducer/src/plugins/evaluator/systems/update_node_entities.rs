use bevy::{
	asset::Assets,
	ecs::system::{Query, ResMut, Single},
	gizmos::gizmos::Gizmos,
	math::Isometry2d,
	sprite::{ColorMaterial, MeshMaterial2d},
	transform::components::Transform,
};

use crate::plugins::evaluator::{
	colours::COLOUR_VISITING,
	components::{debugger_context::DebuggerContextComponent, node::NodeComponent},
};

pub fn update_node_entities(
	mut gizmos: Gizmos,
	context: Single<&DebuggerContextComponent>,
	node_entities: Query<(
		&NodeComponent,
		&mut Transform,
		&mut MeshMaterial2d<ColorMaterial>,
	)>,
	mut materials: ResMut<Assets<ColorMaterial>>,
) {
	let inner_context = context.with_context();
	for (node_component, mut transform, mut material) in node_entities {
		let (colour, coordinates, highlight_radius) =
			context.update_node_entity(&*inner_context, &node_component.node().node);
		let mesh_material_handle = materials.add(ColorMaterial::from_color(colour));
		material.0 = mesh_material_handle;
		transform.translation.x = coordinates.x;
		transform.translation.y = coordinates.y;
		if let Some(highlight_radius) = highlight_radius {
			gizmos.circle_2d(
				Isometry2d::from_translation(coordinates),
				highlight_radius * 0.97,
				COLOUR_VISITING,
			);
		}
	}
}
