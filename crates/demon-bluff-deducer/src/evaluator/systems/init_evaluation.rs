use bevy::{prelude::*, tasks::AsyncComputeTaskPool};
use demon_bluff_logic_engine::predict;

use crate::evaluator::components::{
	game_state::GameStateComponent, prediction::PredictionComponent,
};

pub fn init_evaluation(mut commands: Commands, query: Single<&GameStateComponent>) {
	let thread_pool = AsyncComputeTaskPool::get();
	let game_state = query.game_state().clone();
	let prediction_task = thread_pool.spawn(async move {
		let log = log::logger();
		predict(&log, &game_state)
	});

	commands.spawn(PredictionComponent::new(prediction_task));
}
