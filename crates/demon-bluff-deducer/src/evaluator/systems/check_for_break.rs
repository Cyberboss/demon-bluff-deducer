use bevy::{
	ecs::system::{Commands, ResMut, Single},
	log::{info, warn},
	state::state::NextState,
};
use demon_bluff_logic_engine::Breakpoint;

use crate::evaluator::{
	components::{
		breakpoint::BreakpointComponent, debugger_channels::DebuggerChannels,
		debugger_context::DebuggerContextComponent,
	},
	state::EvaluatorState,
};

pub fn check_for_break(
	mut commands: Commands,
	channels: Single<&DebuggerChannels>,
	mut next_state: ResMut<NextState<EvaluatorState>>,
) {
	if let Some(breakpoint) = match channels.try_get_breakpoint() {
		Ok(breakpoint) => breakpoint,
		Err(_) => panic!("Could not get next breakpoint! Channel unexpectedly closed"),
	} {
		info!("Breakpoint: {}", breakpoint);

		match &breakpoint {
			Breakpoint::Initialize(context) => {
				commands.spawn(DebuggerContextComponent::new(context.clone()));
			}
			_ => warn!("Breakpoint handler unimplemented!"),
		}

		commands.spawn(BreakpointComponent::new(breakpoint));
		next_state.set(EvaluatorState::Break);
	}
}
