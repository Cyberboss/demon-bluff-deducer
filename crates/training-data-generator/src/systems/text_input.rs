use bevy::{
	ecs::{
		entity::Entity,
		event::EventReader,
		query::With,
		system::{Commands, ResMut, Single},
	},
	input::keyboard::{Key, KeyboardInput},
	math::UVec2,
	render::camera::ClearColor,
	text::Text2d,
};

use crate::{
	components::{
		annotating_image::AnnotatingImageComponent, click_point::ClickPoint,
		initial_click_point::InitialClickPoint, second_click_point::SecondClickPoint,
		text_input::TextInputComponent,
	},
	data::word::Word,
};

pub fn text_input(
	mut commands: Commands,
	mut annotator: Single<&mut AnnotatingImageComponent>,
	mut keyboard_input_reader: EventReader<KeyboardInput>,
	initial_click_point: Single<(Entity, &ClickPoint), With<InitialClickPoint>>,
	second_click_point: Single<(Entity, &ClickPoint), With<SecondClickPoint>>,
	edit_text: Single<(Entity, &mut Text2d), With<TextInputComponent>>,
	mut clear_color: ResMut<ClearColor>,
) {
	let (text_entity, mut text) = edit_text.into_inner();
	for keyboard_input in keyboard_input_reader.read() {
		// Only trigger changes when the key is first pressed.
		if !keyboard_input.state.is_pressed() {
			continue;
		}

		match (&keyboard_input.logical_key, &keyboard_input.text) {
			(Key::Enter, _) => {
				if text.is_empty() {
					continue;
				}

				let (entity_1, initial_click_point) = initial_click_point.into_inner();
				let (entity_2, second_click_point) = second_click_point.into_inner();

				let image_dimenisons = annotator.image_size();

				let image_space_coord_1 = UVec2::new(
					image_dimenisons.x / 2 + initial_click_point.point().x.round() as u32,
					image_dimenisons.y / 2 + initial_click_point.point().y.round() as u32,
				);
				let image_space_coord_2 = UVec2::new(
					image_dimenisons.x / 2 + second_click_point.point().y.round() as u32,
					image_dimenisons.y / 2 + second_click_point.point().y.round() as u32,
				);

				let (left_x, right_x) = if image_space_coord_1.x < image_space_coord_2.x {
					(image_space_coord_1.x, image_space_coord_2.x)
				} else {
					(image_space_coord_2.x, image_space_coord_1.x)
				};

				let (top_y, bottom_y) = if image_space_coord_1.y < image_space_coord_2.y {
					(image_space_coord_1.y, image_space_coord_2.y)
				} else {
					(image_space_coord_2.y, image_space_coord_1.y)
				};

				annotator.add_word(Word::new(text.0.clone(), left_x, top_y, right_x, bottom_y));

				commands.entity(entity_1).despawn();
				commands.entity(entity_2).despawn();
				commands.entity(text_entity).despawn();
				clear_color.0 = ClearColor::default().0;

				return;
			}
			(Key::Backspace, _) => {
				text.pop();
			}
			(Key::Space, _) => {
				// words should not have spaces
				continue;
			}
			(_, Some(inserted_text)) => {
				// Make sure the text doesn't have any control characters,
				// which can happen when keys like Escape are pressed
				if inserted_text.chars().all(is_printable_char) {
					text.push_str(inserted_text);
				}
			}
			_ => continue,
		}
	}
}

// this logic is taken from egui-winit:
// https://github.com/emilk/egui/blob/adfc0bebfc6be14cee2068dee758412a5e0648dc/crates/egui-winit/src/lib.rs#L1014-L1024
fn is_printable_char(chr: char) -> bool {
	let is_in_private_use_area = ('\u{e000}'..='\u{f8ff}').contains(&chr)
		|| ('\u{f0000}'..='\u{ffffd}').contains(&chr)
		|| ('\u{100000}'..='\u{10fffd}').contains(&chr);

	!is_in_private_use_area && !chr.is_ascii_control()
}
