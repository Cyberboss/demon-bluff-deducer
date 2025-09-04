use std::fmt::{Display, Formatter};

use super::fmt_desire;
use crate::engine::index_reference::IndexReference;

/// A reference to a [`DesireType`] that a [`Hypothesis`] declares a desire for or not
#[derive(Debug, PartialEq, Eq)]
pub struct DesireProducerReference(usize);

impl IndexReference for DesireProducerReference {
	fn new(index: usize) -> Self {
		Self(index)
	}

	fn clone(&self) -> Self {
		Self(self.0)
	}

	fn index(&self) -> usize {
		self.0
	}
}

impl Display for DesireProducerReference {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		fmt_desire(self.0, f)
	}
}
