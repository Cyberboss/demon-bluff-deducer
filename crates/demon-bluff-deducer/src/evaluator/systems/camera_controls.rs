use bevy::{
	ecs::{
		event::EventReader,
		query::With,
		system::{Query, Res},
	},
	input::{ButtonInput, keyboard::KeyCode, mouse::MouseWheel},
	math::ops::{abs, powf, round},
	render::camera::{Camera, Projection},
	time::{Fixed, Time},
	transform::components::Transform,
};

pub fn camera_controls(
	mut camera_query: Query<(&mut Transform, &mut Projection), With<Camera>>,
	input: Res<ButtonInput<KeyCode>>,
	mut evr_scroll: EventReader<MouseWheel>,
	time: Res<Time<Fixed>>,
) {
	let Ok((mut transform, mut projection)) = camera_query.single_mut() else {
		return;
	};
	let fspeed = 600.0 * time.delta_secs();
	// Camera movement controls
	if input.pressed(KeyCode::KeyW) {
		transform.translation.y += fspeed;
	}
	if input.pressed(KeyCode::KeyS) {
		transform.translation.y -= fspeed;
	}
	if input.pressed(KeyCode::KeyA) {
		transform.translation.x -= fspeed;
	}
	if input.pressed(KeyCode::KeyD) {
		transform.translation.x += fspeed;
	}

	// Camera zoom controls
	if let Projection::Orthographic(projection2d) = &mut *projection {
		for event in evr_scroll.read() {
			let zoom_in = event.y <= 0.0;
			let y_value = round(event.y);
			for _ in 0..(abs(y_value) as i32) {
				projection2d.scale *=
					powf(if zoom_in { 4.0f32 } else { 0.25f32 }, time.delta_secs());
			}
		}
	}
}
