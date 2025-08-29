use bevy::{
	asset::Assets,
	ecs::system::{Commands, ResMut, Single},
	log::warn,
	math::primitives::Circle,
	render::mesh::{Mesh, Mesh2d},
	sprite::{ColorMaterial, MeshMaterial2d},
	transform::components::Transform,
};
use demon_bluff_logic_engine::Breakpoint;

use crate::plugins::evaluator::{
	colours::COLOUR_NEUTRAL,
	components::{
		breakpoint::BreakpointComponent, debugger_context::DebuggerContextComponent,
		node::NodeComponent,
	},
	node_radius::NodeRadius,
};

pub fn update_graph_from_breakpoint(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<ColorMaterial>>,
	context: Single<&mut DebuggerContextComponent>,
	breakpoint: Single<&BreakpointComponent>,
) {
	let mut context = context.into_inner();
	match breakpoint.breakpoint() {
		Breakpoint::Initialize(_) => {}
		Breakpoint::RegisterHypothesis(index, root) => {
			let node = context.register_hypothesis(*index, *root);

			commands.spawn((
				NodeComponent::new(node.user_data.clone()),
				Mesh2d(meshes.add(Circle::new(node.radius(*root)))),
				MeshMaterial2d(materials.add(COLOUR_NEUTRAL)),
				Transform::from_xyz(node.x, node.y, 0.0),
			));
		}
		Breakpoint::RegisterDesire(index) => context.register_desire(*index),
		Breakpoint::IterationStart(iteration) => {
			if *iteration == 1 {
				context.finalize_edges();
			}
		}
		Breakpoint::DesireUpdate(producer_hypothesis_index, desire_index, desired) => {
			context.update_desire_producer(*producer_hypothesis_index, *desire_index, *desired)
		}
		_ => {
			warn!(
				"Unhandled graph update from breakpoint: {}",
				breakpoint.breakpoint()
			);
		}
	}
}
