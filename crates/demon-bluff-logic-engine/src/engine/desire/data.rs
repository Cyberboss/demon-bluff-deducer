use std::{
    collections::HashSet,
    fmt::{Display, Formatter},
};

use serde::Serialize;

use crate::engine::HypothesisReference;

#[derive(Debug, PartialEq, Eq, Serialize)]
struct DesireData {
    pending: HashSet<HypothesisReference>,
    desired: HashSet<HypothesisReference>,
    undesired: HashSet<HypothesisReference>,
}

impl DesireData {
    fn total(&self) -> usize {
        self.undesired.len() + self.pending.len() + self.desired.len()
    }
}

impl Display for DesireData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.desired.len(), self.total())?;

        if self.pending.len() > 0 {
            write!(f, " ({} Pending)", self.pending.len())
        } else {
            Ok(())
        }
    }
}
