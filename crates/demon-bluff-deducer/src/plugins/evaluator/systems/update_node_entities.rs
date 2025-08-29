use bevy::{
	asset::Assets,
	ecs::system::{Query, ResMut, Single},
	sprite::{ColorMaterial, MeshMaterial2d},
	transform::components::Transform,
};

use crate::plugins::evaluator::components::{
	debugger_context::DebuggerContextComponent, node::NodeComponent,
};

pub fn update_node_entities(
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
		let (colour, coordinates) =
			context.update_node_entity(&*inner_context, node_component.node());
		let mesh_material_handle = materials.add(ColorMaterial::from_color(colour));
		material.0 = mesh_material_handle;
		transform.translation.x = coordinates.x;
		transform.translation.y = coordinates.y;
	}
}
