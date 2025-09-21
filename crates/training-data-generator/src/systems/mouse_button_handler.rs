use bevy::{
	asset::Assets,
	color::{Color, LinearRgba},
	core_pipeline::core_2d::Camera2d,
	ecs::{
		entity::Entity,
		query::With,
		system::{Commands, Res, ResMut, Single},
	},
	image::Image,
	input::{ButtonInput, mouse::MouseButton},
	log::{error, warn},
	math::{Rect, vec2},
	render::camera::{Camera, ClearColor},
	sprite::Sprite,
	text::{Text2d, TextFont},
	transform::components::{GlobalTransform, Transform},
	window::{PrimaryWindow, Window},
};

use crate::components::{
	annotating_image::{self, AnnotatingImageComponent},
	click_point::ClickPoint,
	initial_click_point::InitialClickPoint,
	second_click_point::SecondClickPoint,
	text_input::TextInputComponent,
};

pub fn mouse_button_handler(
	mut commands: Commands,
	initial_click_point: Option<Single<(Entity, &ClickPoint), With<InitialClickPoint>>>,
	second_click_point: Option<Single<(Entity, &ClickPoint), With<SecondClickPoint>>>,
	window: Single<&Window, With<PrimaryWindow>>,
	annotating_image: Single<&AnnotatingImageComponent>,
	text_input: Option<Single<Entity, With<TextInputComponent>>>,
	camera: Single<(&Camera, &GlobalTransform), With<Camera2d>>,
	buttons: Res<ButtonInput<MouseButton>>,
	mut clear_color: ResMut<ClearColor>,
) {
	if buttons.just_pressed(MouseButton::Right) {
		if let Some(second_click_point) = second_click_point {
			let (entity, _) = second_click_point.into_inner();
			commands.entity(entity).despawn();
			clear_color.0 = ClearColor::default().0;
			if let Some(text_input) = text_input {
				commands.entity(text_input.into_inner()).despawn();
			} else {
				error!("Second click point without text!?");
			}
		} else if let Some(initial_click_point) = initial_click_point {
			let (entity, _) = initial_click_point.into_inner();
			commands.entity(entity).despawn();
		}
	} else if buttons.just_pressed(MouseButton::Left) && second_click_point.is_none() {
		let Some(cursor_position) = window.cursor_position() else {
			return;
		};

		let (camera, camera_transform) = camera.into_inner();

		let Ok(cursor_coord) = camera.viewport_to_world_2d(camera_transform, cursor_position)
		else {
			return;
		};

		let image_dimensions = annotating_image.image_size();

		let image_rect = Rect::new(
			-((image_dimensions.x / 2) as f32),
			-((image_dimensions.y / 2) as f32),
			(image_dimensions.x / 2) as f32,
			(image_dimensions.y / 2) as f32,
		);
		if !image_rect.contains(cursor_coord) {
			warn!("Cursor is outside bounds of image! Ignoring click");
			return;
		}

		if initial_click_point.is_some() {
			clear_color.0 = Color::LinearRgba(LinearRgba::rgb(0.0, 0.0, 1.0));
			commands.spawn((SecondClickPoint, ClickPoint::new(cursor_coord)));
			commands.spawn((
				TextInputComponent,
				Text2d::default(),
				TextFont {
					font_size: 100.0,
					..Default::default()
				},
			));
		} else {
			commands.spawn((InitialClickPoint, ClickPoint::new(cursor_coord)));
		}
	}
}
