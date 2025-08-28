use std::collections::HashSet;

use bevy::{ecs::component::Component, tasks::Task};
use demon_bluff_logic_engine::{PlayerAction, PredictionError};

#[derive(Component)]
pub struct PredictionComponent {
	task: Task<Result<HashSet<PlayerAction>, PredictionError>>,
}

impl PredictionComponent {
	pub fn new(task: Task<Result<HashSet<PlayerAction>, PredictionError>>) -> Self {
		Self { task }
	}
}
