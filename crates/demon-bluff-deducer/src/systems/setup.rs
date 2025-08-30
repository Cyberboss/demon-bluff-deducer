use bevy::{
	core_pipeline::core_2d::Camera2d,
	ecs::system::{Commands, Single},
	render::camera::{OrthographicProjection, Projection, ScalingMode},
	window::Window,
};

pub fn setup(mut commands: Commands, window: Single<&Window>) {
	let window = window.into_inner();
	commands.spawn((
		Camera2d,
		Projection::from(OrthographicProjection {
			scaling_mode: ScalingMode::Fixed {
				width: window.width(),
				height: window.height(),
			},
			..OrthographicProjection::default_2d()
		}),
	));
}
