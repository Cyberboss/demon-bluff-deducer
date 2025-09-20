use bevy::{
	asset::{Assets, RenderAssetUsages},
	ecs::{
		entity::Entity,
		system::{Commands, Res, ResMut, Single},
	},
	image::Image,
	input::{ButtonInput, keyboard::KeyCode},
	sprite::Sprite,
};
use image::DynamicImage;

use crate::{
	components::annotating_image::AnnotatingImageComponent, data::image_id::ImageId,
	resources::window::WindowResource,
};

pub fn keyboard_handler(
	mut commands: Commands,
	mut assets: ResMut<Assets<Image>>,
	current_annotation: Option<Single<(Entity, &mut AnnotatingImageComponent)>>,
	window: Res<WindowResource>,
	keyboard_input: Res<ButtonInput<KeyCode>>,
) {
	if keyboard_input.just_pressed(KeyCode::KeyN) {
		let next_image_id;
		if let Some(current_annotation) = current_annotation {
			let (current_annotation_entity, mut current_annotation) =
				current_annotation.into_inner();

			next_image_id = current_annotation.complete_and_get_next_image_id();

			commands.entity(current_annotation_entity).despawn();
		} else {
			next_image_id = ImageId::new();
		}

		let raw_image = window
			.take_screenshot()
			.expect("Failed to take screenshot!");

		let dynamic_image = DynamicImage::from(raw_image.clone());

		let bevy_image = Image::from_dynamic(dynamic_image, false, RenderAssetUsages::RENDER_WORLD);

		let asset_handle = assets.add(bevy_image);

		let sprite = Sprite::from_image(asset_handle);

		commands.spawn((
			AnnotatingImageComponent::new(raw_image, next_image_id),
			sprite,
		));
	}
}
