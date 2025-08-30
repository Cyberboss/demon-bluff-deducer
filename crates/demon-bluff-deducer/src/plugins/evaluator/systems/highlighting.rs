use bevy::{
	ecs::{
		observer::Trigger,
		query::With,
		system::{Commands, Query, Single},
	},
	gizmos::gizmos::Gizmos,
	picking::events::{Out, Over, Pointer},
};

use crate::plugins::evaluator::components::{
	debugger_context::DebuggerContextComponent, node::NodeComponent,
	node_highlighted::NodeHighlightedComponent,
};

pub fn start_highlight_node(trigger: Trigger<Pointer<Over>>, mut commands: Commands) {
	commands
		.entity(trigger.target)
		.insert(NodeHighlightedComponent);
}

pub fn stop_highlight_node(trigger: Trigger<Pointer<Out>>, mut commands: Commands) {
	commands
		.entity(trigger.target)
		.remove::<NodeHighlightedComponent>();
}

pub fn draw_highlights(
	mut gizmos: Gizmos,
	context: Single<&DebuggerContextComponent>,
	pointed_nodes: Query<&NodeComponent, With<NodeHighlightedComponent>>,
) {
	let context = context.into_inner();
	for node in pointed_nodes {
		context.draw_highlight(node.node(), &mut gizmos);
	}
}
