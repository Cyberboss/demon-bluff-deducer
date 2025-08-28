use std::fmt::{Display, Formatter};

use super::HypothesisReference;

#[derive(Debug, PartialEq, Eq, Hash)]
pub(super) struct Cycle {
    order_from_root: Vec<HypothesisReference>,
}

impl Cycle {
    pub fn new(order_from_root: Vec<HypothesisReference>) -> Self {
        Self { order_from_root }
    }

    pub fn references(&self) -> &Vec<HypothesisReference> {
        &self.order_from_root
    }
}

impl Display for Cycle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut first = true;
        for reference in &self.order_from_root {
            if first {
                first = false;
            } else {
                write!(f, " -> ")?;
            }

            write!(f, "{reference}")?;
        }

        Ok(())
    }
}
