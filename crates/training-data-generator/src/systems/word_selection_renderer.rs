use bevy::{
	color::{Color, LinearRgba},
	core_pipeline::core_2d::Camera2d,
	ecs::{query::With, system::Single},
	gizmos::gizmos::Gizmos,
	math::{Isometry2d, Vec2, ops::abs},
	render::camera::Camera,
	transform::components::GlobalTransform,
	window::{PrimaryWindow, Window},
};

use crate::components::{
	click_point::ClickPoint, initial_click_point::InitialClickPoint,
	second_click_point::SecondClickPoint,
};

pub fn word_selection_renderer(
	mut gizmos: Gizmos,
	initial_click_point: Single<&ClickPoint, With<InitialClickPoint>>,
	second_click_point: Option<Single<&ClickPoint, With<SecondClickPoint>>>,
	window: Single<&Window, With<PrimaryWindow>>,
	camera: Single<(&Camera, &GlobalTransform), With<Camera2d>>,
) {
	let Some(cursor_position) = window.cursor_position() else {
		return;
	};

	let coord_1 = match &second_click_point {
		Some(second_click_point) => *second_click_point.point(),
		None => {
			let (camera, camera_transform) = camera.into_inner();

			let Ok(coord_1) = camera.viewport_to_world_2d(camera_transform, cursor_position) else {
				return;
			};

			coord_1
		}
	};
	let coord_2 = initial_click_point.point();

	let center = (coord_1 + coord_2) / 2.0;
	let size = Vec2::new(abs(coord_1.x - coord_2.x), abs(coord_1.y - coord_2.y));

	gizmos.rect_2d(
		Isometry2d::from_translation(center),
		size,
		Color::LinearRgba(if second_click_point.is_some() {
			LinearRgba::rgb(0.0, 0.0, 1.0)
		} else {
			LinearRgba::rgb(1.0, 0.0, 0.0)
		}),
	);
}
