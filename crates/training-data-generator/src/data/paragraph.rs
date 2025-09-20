use std::cmp::{max, min};

use serde::Serialize;

use super::{
	get_bounding_verticies::get_bounding_verticies,
	line::{Line, LineBuilder},
	verticies::Verticies,
};

#[derive(Serialize)]
pub(super) struct Paragraph {
	verticies: Verticies,
	legible: bool,
	lines: Vec<Line>,
}

pub struct ParagraphBuilder {
	inner: Paragraph,
}

impl ParagraphBuilder {
	pub fn new() -> Self {
		Self {
			inner: Paragraph {
				verticies: Verticies::default(),
				legible: true,
				lines: Vec::new(),
			},
		}
	}

	pub fn add_line(&mut self, line: LineBuilder) {
		self.inner.lines.push(line.build());
	}

	pub fn build(self) -> Paragraph {
		let mut paragraph = self.inner;
		paragraph.verticies = get_bounding_verticies(&paragraph.lines);
		paragraph
	}
}
