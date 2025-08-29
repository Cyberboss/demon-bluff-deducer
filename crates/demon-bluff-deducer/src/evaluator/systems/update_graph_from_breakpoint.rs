use bevy::{ecs::system::Single, log::warn, text::cosmic_text::ttf_parser::gpos::SingleAdjustment};
use demon_bluff_logic_engine::{Breakpoint, DebuggerContext};

use crate::evaluator::components::{
	breakpoint::{self, BreakpointComponent},
	debugger_channels::DebuggerChannels,
	debugger_context::DebuggerContextComponent,
};

pub fn update_graph_from_breakpoint(
	context: Single<&mut DebuggerContextComponent>,
	channels: Single<&mut DebuggerChannels>,
	breakpoint: Single<&BreakpointComponent>,
) {
	let mut context = context.into_inner();
	match breakpoint.breakpoint() {
		Breakpoint::Initialize(_) => {}
		Breakpoint::RegisterHypothesis(index, root) => context.register_hypothesis(*index, *root),
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
