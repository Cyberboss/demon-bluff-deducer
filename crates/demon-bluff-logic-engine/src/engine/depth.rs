use std::fmt::{Display, Formatter};

use super::HypothesisReference;

pub struct Depth {
    depth: usize,
    reference: Option<HypothesisReference>,
}

impl Depth {
    pub fn new(depth: usize, reference: Option<HypothesisReference>) -> Self {
        Self { depth, reference }
    }
}

impl Display for Depth {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for _ in 0..self.depth {
            write!(f, "  ")?
        }

        if let Some(reference) = self.reference.as_ref() {
            write!(f, "- [{}]", reference)
        } else {
            Ok(())
        }
    }
}
