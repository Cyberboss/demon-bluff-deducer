use image::RgbaImage;
use serde::Serialize;

use super::{
	image_id::ImageId,
	paragraph::{Paragraph, ParagraphBuilder},
};

#[derive(Serialize)]
pub struct Annotation {
	image_id: String,
	image_width: u32,
	image_height: u32,
	paragraphs: Vec<Paragraph>,
}

pub struct AnnotationBuilder {
	inner: Annotation,
}

impl AnnotationBuilder {
	pub fn new(image_id: &ImageId, image: &RgbaImage) -> Self {
		Self {
			inner: Annotation {
				image_id: format!("{:07}", image_id.id()),
				image_width: image.width(),
				image_height: image.height(),
				paragraphs: Vec::new(),
			},
		}
	}

	pub fn add_paragraph(&mut self, paragraph: ParagraphBuilder) {
		self.inner.paragraphs.push(paragraph.build());
	}

	pub fn build(self) -> Annotation {
		self.inner
	}
}
