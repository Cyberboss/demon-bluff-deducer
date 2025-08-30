use bevy::{
	ecs::{
		entity::Entity,
		query::With,
		system::{Commands, Res, Single},
	},
	input::{ButtonInput, keyboard::KeyCode},
};

use crate::plugins::evaluator::components::help_text::HelpTextComponent;

pub fn remove_help_text(
	mut commands: Commands,
	input: Res<ButtonInput<KeyCode>>,
	help_text: Single<Entity, With<HelpTextComponent>>,
) {
	if input.just_pressed(KeyCode::Escape) {
		let entity = help_text.into_inner();
		commands.entity(entity).despawn();
	}
}
