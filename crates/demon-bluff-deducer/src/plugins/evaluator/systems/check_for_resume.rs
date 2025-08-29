use bevy::{
	ecs::{
		entity::Entity,
		query::With,
		system::{Commands, Res, ResMut, Single},
	},
	input::{ButtonInput, keyboard::KeyCode},
	log::info,
	state::state::NextState,
};

use crate::plugins::evaluator::{
	components::{
		breakpoint::BreakpointComponent, debugger_channels::DebuggerChannels,
	},
	state::EvaluatorState,
};

pub fn check_for_resume(
	mut commands: Commands,
	channels: Single<&DebuggerChannels>,
	breakpoint: Single<Entity, With<BreakpointComponent>>,
	keys: Res<ButtonInput<KeyCode>>,
	mut next_state: ResMut<NextState<EvaluatorState>>,
) {
	if true || keys.just_pressed(KeyCode::Space) {
		info!("Resuming from breakpoint");
		commands.entity(*breakpoint).despawn();
		channels.send_continue();
		next_state.set(EvaluatorState::Running);
	}
}
