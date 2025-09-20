use std::{fs::OpenOptions, io::Write};

use bevy::ecs::component::Component;
use image::RgbaImage;
use owned_drop::{DropOwned, OwnedDroppable};

use crate::data::{
	annotation::AnnotationBuilder, image_id::ImageId, line::LineBuilder,
	paragraph::ParagraphBuilder,
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
			}),
		}
	}

	pub fn next_image_id(&self) -> ImageId {
		let mut next_image_id = self.inner.image_id.clone();
		next_image_id.inc();
		next_image_id
	}
}

impl OwnedDroppable for AnnotatingImageBuilder {
	fn drop_owned(self) {
		let built_annotation = self.annotation.build();
		let json =
			serde_json::to_string(&built_annotation).expect("Serialization of annotation failed!");

		let path = self.image_id.path();
		let mut file = OpenOptions::new()
			.write(true)
			.append(true)
			.open(&path)
			.unwrap_or_else(|error| {
				panic!("Failed to open {} for writing: {}", path.display(), error)
			});

		writeln!(file, "{}", json).expect("Failed to write json to file!");
	}
}
