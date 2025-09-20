use bevy::{
	core_pipeline::core_2d::Camera2d,
	ecs::{
		query::With,
		system::{Commands, Single},
	},
	transform::components::Transform,
};

use crate::components::annotating_image::AnnotatingImageComponent;

pub fn mouse_button_handler(
	mut commands: Commands,
	camera: Single<&Transform, With<Camera2d>>,
	annotator: Single<&mut AnnotatingImageComponent>,
) {
}
