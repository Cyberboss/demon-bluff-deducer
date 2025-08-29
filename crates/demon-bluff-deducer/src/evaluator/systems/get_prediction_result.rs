use bevy::{
	ecs::{
		entity::Entity,
		system::{Commands, Single},
	},
	tasks::{block_on, poll_once},
};

use crate::evaluator::components::prediction::PredictionComponent;

pub fn get_prediction_result(
	mut commands: Commands,
	task: Single<(Entity, &mut PredictionComponent)>,
) {
	let (entity, mut task) = task.into_inner();
	if let Some(_) = block_on(poll_once(task.task_mut())) {
		commands.entity(entity).despawn();
	}
}
