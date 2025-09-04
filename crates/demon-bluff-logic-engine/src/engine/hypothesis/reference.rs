use std::fmt::{Display, Formatter};

use serde::Serialize;

use crate::engine::index_reference::IndexReference;

/// A reference to a `Hypothesis` in the graph.
#[derive(Debug, PartialEq, Eq, Hash, Serialize)]
pub struct HypothesisReference(usize);

impl IndexReference for HypothesisReference {
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

impl Display for HypothesisReference {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "H-{:05}", self.0 + 1)
	}
}
