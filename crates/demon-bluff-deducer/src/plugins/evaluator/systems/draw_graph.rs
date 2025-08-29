use bevy::{
	ecs::system::{Res, Single},
	gizmos::gizmos::Gizmos,
	time::Time,
};

use crate::plugins::evaluator::components::debugger_context::DebuggerContextComponent;

pub fn draw_graph(gizmos: Gizmos, time: Res<Time>, context: Single<&mut DebuggerContextComponent>) {
	context.into_inner().update_and_draw_graph(gizmos, &time);
}
