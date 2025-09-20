use serde::Serialize;

use super::{has_verticies::HasVerticies, verticies::Verticies};

#[derive(Serialize)]
pub struct Word {
	verticies: Verticies,
	text: String,
	legible: bool,
	handwritten: bool,
	vertical: bool,
}

impl HasVerticies for Word {
	fn verticies(&self) -> &Verticies {
		&self.verticies
	}
}

impl Word {
	pub fn new(text: String, left_x: u32, top_y: u32, right_x: u32, bottom_y: u32) -> Self {
		Self {
			verticies: [
				[left_x, top_y],
				[right_x, top_y],
				[right_x, bottom_y],
				[left_x, bottom_y],
			],
			text,
			legible: true,
			handwritten: false,
			vertical: false,
		}
	}

	pub fn text(&self) -> &String {
		&self.text
	}
}
