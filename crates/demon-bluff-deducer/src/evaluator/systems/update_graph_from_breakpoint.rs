use bevy::{ecs::system::Single, log::warn, text::cosmic_text::ttf_parser::gpos::SingleAdjustment};
use demon_bluff_logic_engine::{Breakpoint, DebuggerContext};

use crate::evaluator::components::{
	breakpoint::{self, BreakpointComponent},
	debugger_context::DebuggerContextComponent,
};

pub fn update_graph_from_breakpoint(
	mut context: Single<&mut DebuggerContextComponent>,
	breakpoint: Single<&BreakpointComponent>,
) {
	let mut context = context.into_inner();
	match breakpoint.breakpoint() {
		Breakpoint::Initialize(_) => {}
		Breakpoint::RegisterHypothesis(index) => context.register_hypothesis(*index),
		Breakpoint::RegisterDesire(index) => context.register_desire(*index),
		Breakpoint::IterationStart(iteration) => {
			if *iteration == 1 {
				context.register_edges();
			}
		}
		_ => warn!(
			"Unhandled graph update from breakpoint: {}",
			breakpoint.breakpoint()
		),
	}
}
