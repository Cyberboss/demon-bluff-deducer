use bevy::{
	core_pipeline::core_2d::Camera2d,
	ecs::{
		query::With,
		system::{Commands, Single},
	},
	window::{PrimaryWindow, Window},
};

use crate::components::annotating_image::AnnotatingImageComponent;

pub fn setup(mut commands: Commands, window: Single<&mut Window, With<PrimaryWindow>>) {
	window.into_inner().set_maximized(true);
	commands.spawn(Camera2d);
}
