use bevy::{
	ecs::system::{Res, Single},
	time::Time,
};

use crate::plugins::evaluator::components::debugger_context::DebuggerContextComponent;

pub fn update_graph(time: Res<Time>, context: Single<&mut DebuggerContextComponent>) {
	context.into_inner().update_graph(&time);
}
