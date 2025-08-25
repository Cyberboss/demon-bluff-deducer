use std::fmt::{Display, Formatter};

use super::r#trait::Desire;

#[derive(Debug)]
pub struct DesireDefinition<TDesire>
where
    TDesire: Desire,
{
    desire: TDesire,
    count: usize,
    used: bool,
}

impl<TDesire> DesireDefinition<TDesire>
where
    TDesire: Desire,
{
    pub fn new(desire: TDesire, count: usize, used: bool) -> Self {
        Self {
            desire,
            count,
            used,
        }
    }
}

impl<TDesire> Display for DesireDefinition<TDesire>
where
    TDesire: Desire,
{
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
