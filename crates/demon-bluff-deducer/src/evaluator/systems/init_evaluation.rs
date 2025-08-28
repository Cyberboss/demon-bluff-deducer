use bevy::{
	prelude::*,
	tasks::{AsyncComputeTaskPool, block_on},
};
use demon_bluff_logic_engine::{Breakpoint, predict, predict_with_debugger};

use crate::evaluator::components::{
	debugger_channels::DebuggerChannels, game_state::GameStateComponent,
	prediction::PredictionComponent,
};

pub fn init_evaluation(mut commands: Commands, query: Single<&GameStateComponent>) {
	let thread_pool = AsyncComputeTaskPool::get();
	let game_state = query.game_state().clone();
	let (channels, continue_receiver, breakpoint_sender) = DebuggerChannels::new();
	let prediction_task = thread_pool.spawn(async move {
		let log = log::logger();
		predict_with_debugger(&log, &game_state, |breakpoint| {
			// TODO: make entire logic stack async
			if let Ok(()) = block_on(breakpoint_sender.send(breakpoint)) {
				_ = block_on(continue_receiver.recv());
			}
		})
	});

	commands.spawn((channels, PredictionComponent::new(prediction_task)));
}
