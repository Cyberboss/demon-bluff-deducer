use bevy::{
	ecs::{
		entity::Entity,
		system::{Commands, Single},
	},
	tasks::{block_on, poll_once},
};

use crate::plugins::evaluator::components::prediction::PredictionComponent;

pub fn get_prediction_result(
	mut commands: Commands,
	task: Single<(Entity, &mut PredictionComponent)>,
) {
	let (entity, mut task) = task.into_inner();
	if block_on(poll_once(task.task_mut())).is_some() {
		commands.entity(entity).despawn();
	}
}
