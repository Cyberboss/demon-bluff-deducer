use serde::Serialize;

use super::{
	get_bounding_verticies::get_bounding_verticies, has_verticies::HasVerticies,
	verticies::Verticies, word::Word,
};

#[derive(Serialize)]
pub(super) struct Line {
	verticies: Verticies,
	text: String,
	legible: bool,
	handwritten: bool,
	vertical: bool,
	words: Vec<Word>,
}

pub struct LineBuilder {
	inner: Line,
}

impl HasVerticies for Line {
	fn verticies(&self) -> &Verticies {
		&self.verticies
	}
}

impl LineBuilder {
	pub fn new() -> Self {
		LineBuilder {
			inner: Line {
				verticies: Verticies::default(),
				text: String::new(),
				legible: true,
				handwritten: false,
				vertical: false,
				words: Vec::new(),
			},
		}
	}

	pub fn add_word(&mut self, word: Word) {
		self.inner.words.push(word);
	}

	pub fn build(self) -> Line {
		let mut line = self.inner;
		line.text = line
			.words
			.iter()
			.map(|word| word.text().as_str())
			.collect::<Vec<&str>>()
			.join(" ");

		line.verticies = get_bounding_verticies(&line.words);

		line
	}
}
