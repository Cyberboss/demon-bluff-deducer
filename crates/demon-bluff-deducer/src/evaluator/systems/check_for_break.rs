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
	if let Ok(Some(breakpoint)) = channels.try_get_breakpoint() {
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
