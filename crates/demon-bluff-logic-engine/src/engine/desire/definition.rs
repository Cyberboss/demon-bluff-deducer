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

	pub fn count(&self) -> usize {
		self.count
	}

	pub fn used(&self) -> bool {
		self.used
	}

	pub fn desire(&self) -> &TDesire {
		&self.desire
	}
}

impl<TDesire> Display for DesireDefinition<TDesire>
where
	TDesire: Desire + Display,
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
