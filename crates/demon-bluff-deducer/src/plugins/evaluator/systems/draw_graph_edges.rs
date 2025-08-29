use bevy::{ecs::system::Single, gizmos::gizmos::Gizmos};

use crate::plugins::evaluator::components::debugger_context::DebuggerContextComponent;

pub fn draw_graph_edges(gizmos: Gizmos, context: Single<&DebuggerContextComponent>) {
	context.into_inner().draw_edges(gizmos);
}
