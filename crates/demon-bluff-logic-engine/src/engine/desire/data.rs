use std::{
	collections::HashSet,
	fmt::{Display, Formatter},
};

use serde::Serialize;

use crate::engine::HypothesisReference;

#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct DesireData {
	pub pending: HashSet<HypothesisReference>,
	pub desired: HashSet<HypothesisReference>,
	pub undesired: HashSet<HypothesisReference>,
}

impl DesireData {
	pub fn total(&self) -> usize {
		self.undesired.len() + self.pending.len() + self.desired.len()
	}
}

impl Display for DesireData {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}/{}", self.desired.len(), self.total())?;

		if !self.pending.is_empty() {
			write!(f, " ({} Pending)", self.pending.len())
		} else {
			Ok(())
		}
	}
}
