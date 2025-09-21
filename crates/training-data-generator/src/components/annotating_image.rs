use std::{fs::OpenOptions, io::Write, mem::replace, path::PathBuf};

use bevy::{
	ecs::component::Component,
	math::{UVec2, Vec2},
};
use image::{RgbImage, RgbaImage, buffer::ConvertBuffer};
use owned_drop::{DropOwned, OwnedDroppable};

use crate::data::{
	annotation::AnnotationBuilder, image_id::ImageId, line::LineBuilder,
	paragraph::ParagraphBuilder, word::Word,
};

#[derive(Component)]
pub struct AnnotatingImageComponent {
	inner: DropOwned<AnnotatingImageBuilder>,
}

struct AnnotatingImageBuilder {
	image: RgbaImage,
	image_id: ImageId,
	annotation: AnnotationBuilder,
	paragraph: ParagraphBuilder,
	line: LineBuilder,
	save_on_drop: bool,
}

impl AnnotatingImageComponent {
	pub fn new(image: RgbaImage, image_id: ImageId) -> Self {
		let annotation = AnnotationBuilder::new(&image_id, &image);
		Self {
			inner: DropOwned::new(AnnotatingImageBuilder {
				image,
				annotation,
				paragraph: ParagraphBuilder::new(),
				line: LineBuilder::new(),
				image_id,
				save_on_drop: false,
			}),
		}
	}

	pub fn reset(&mut self) {
		self.inner.annotation = AnnotationBuilder::new(&self.inner.image_id, &self.inner.image);
		self.inner.paragraph = ParagraphBuilder::new();
		self.inner.line = LineBuilder::new();
	}

	pub fn image_size(&self) -> UVec2 {
		UVec2::new(self.inner.image.width(), self.inner.image.height())
	}

	pub fn complete_and_get_next_image_id(&mut self) -> ImageId {
		self.inner.save_on_drop = true;
		let mut next_image_id = self.inner.image_id.clone();
		next_image_id.inc();
		next_image_id
	}

	pub fn add_word(&mut self, word: Word) {
		self.inner.line.add_word(word);
	}

	pub fn finish_line(&mut self) {
		let line = replace(&mut self.inner.line, LineBuilder::new());
		self.inner.paragraph.add_line(line);
	}

	pub fn finish_paragraph(&mut self) {
		let paragraph = replace(&mut self.inner.paragraph, ParagraphBuilder::new());
		self.inner.annotation.add_paragraph(paragraph);
	}
}

impl OwnedDroppable for AnnotatingImageBuilder {
	fn drop_owned(self) {
		if !self.save_on_drop {
			return;
		}

		let built_annotation = self.annotation.build();
		let json =
			serde_json::to_string(&built_annotation).expect("Serialization of annotation failed!");

		ConvertBuffer::<RgbImage>::convert(&self.image)
			.save(self.image_id.jpg_path())
			.unwrap_or_else(|error| panic!("Failed to save screenshot: {}", error));

		let json_path = PathBuf::from("S:/workspace/demon-bluff-deducer/dataset/train.jsonl");
		let mut file = OpenOptions::new()
			.write(true)
			.append(true)
			.create(true)
			.open(&json_path)
			.unwrap_or_else(|error| {
				panic!(
					"Failed to open {} for writing: {}",
					json_path.display(),
					error
				)
			});

		writeln!(file, "{}", json).expect("Failed to write json to file!");
	}
}
