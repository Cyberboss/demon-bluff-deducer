use std::{collections::BTreeSet, fmt::Display, hash::Hash};

use demon_bluff_gameplay_engine::villager::VillagerIndex;
use serde::Serialize;

#[derive(Debug, Eq, Clone, Serialize, PartialOrd, Ord)]
pub struct AbilityAttempt {
	source: VillagerIndex,
	targets: BTreeSet<VillagerIndex>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize)]
pub enum PlayerAction {
	TryReveal(VillagerIndex),
	TryExecute(VillagerIndex),
	Ability(AbilityAttempt),
}

impl AbilityAttempt {
	pub fn new(source: VillagerIndex, targets: BTreeSet<VillagerIndex>) -> Self {
		Self { source, targets }
	}

	pub fn source(&self) -> &VillagerIndex {
		&self.source
	}

	pub fn targets(&self) -> &BTreeSet<VillagerIndex> {
		&self.targets
	}
}

impl Display for AbilityAttempt {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "Use {}'s ability on ", self.source)?;

		let mut first = true;
		for target in self.targets.iter() {
			if first {
				first = false;
			} else {
				write!(f, ", ")?
			}

			write!(f, "{}", target)?
		}

		Ok(())
	}
}

impl PartialEq for AbilityAttempt {
	fn eq(&self, other: &Self) -> bool {
		self.source == other.source
			&& self.targets.len() == other.targets.len()
			&& self
				.targets
				.iter()
				.all(|target| other.targets.contains(target))
	}
}

impl Hash for AbilityAttempt {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		self.source.hash(state);
		let mut targets: Vec<VillagerIndex> = self.targets.iter().cloned().collect();
		targets.sort();
		targets.hash(state);
	}
}

impl Display for PlayerAction {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::TryReveal(villager_index) => write!(f, "Reveal {villager_index}"),
			Self::TryExecute(villager_index) => write!(f, "Execute {villager_index}"),
			Self::Ability(ability_attempt) => {
				write!(f, "Use {}'s ability on ", ability_attempt.source,)?;

				let length = ability_attempt.targets.len();
				for (index, target) in ability_attempt.targets.iter().enumerate() {
					if index != 0 {
						write!(f, ", ")?;
					}

					if index == length && length > 1 {
						write!(f, "and {target}")?;
					} else {
						write!(f, "{target}")?
					}
				}

				Ok(())
			}
		}
	}
}
