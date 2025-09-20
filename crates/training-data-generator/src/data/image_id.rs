use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct ImageId {
	counter: u32,
}

impl ImageId {
	pub fn new() -> Self {
		let mut this = Self { counter: 0 };

		if this.has_collision() {
			this.inc();
		}

		this
	}

	pub fn inc(&mut self) {
		loop {
			self.counter += 1;
			if !self.has_collision() {
				break;
			}
		}
	}

	pub fn id(&self) -> u32 {
		self.counter
	}

	pub fn path(&self) -> PathBuf {
		PathBuf::from(format!(
			"X:/workspace/demon-bluff-deducer/dataset/images/train/{:07}.jpg",
			self.counter
		))
	}

	fn has_collision(&self) -> bool {
		self.path().exists()
	}
}
