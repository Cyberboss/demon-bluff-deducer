use std::fmt::{Display, Formatter};

use crate::hypotheses::desires::DesireType;

#[derive(Debug)]
pub struct DesireDefinition<TDesire> {
    desire: TDesire,
    count: usize,
    used: bool,
}

impl<TDesire> DesireDefinition<TDesire> {
    pub fn new(desire: TDesire, count: usize, used: bool) -> Self {
        Self {
            desire,
            count,
            used,
        }
    }
}

impl<TDesire> Display for DesireDefinition<TDesire> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} ({} Producer(s)){}",
            self.desire,
            self.count,
            if self.used { "" } else { " (UNUSED)" }
        )
    }
}
