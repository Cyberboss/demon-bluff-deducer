use bevy::{ecs::component::Component, math::Vec2};

#[derive(Component)]
pub struct ClickPoint {
	point: Vec2,
}

impl ClickPoint {
	pub fn new(point: Vec2) -> Self {
		Self { point }
	}

	pub fn point(&self) -> &Vec2 {
		&self.point
	}
}
