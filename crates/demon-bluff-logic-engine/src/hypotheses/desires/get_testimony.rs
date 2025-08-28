use std::fmt::{Formatter, Result};

use demon_bluff_gameplay_engine::villager::VillagerIndex;

use crate::engine::Desire;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct GetTestimonyDesire {
	index: VillagerIndex,
}

impl GetTestimonyDesire {
	pub fn new(index: VillagerIndex) -> Self {
		Self { index }
	}
}

impl Desire for GetTestimonyDesire {
	fn describe(&self, f: &mut Formatter<'_>) -> Result {
		write!(f, "Get {}'s Testimony", self.index)
	}
}
