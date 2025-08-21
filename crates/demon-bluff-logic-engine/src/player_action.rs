use std::collections::HashSet;
use std::hash::Hash;

use demon_bluff_gameplay_engine::villager::VillagerIndex;

#[derive(Debug, Eq)]
pub struct AbilityAttempt {
    source: VillagerIndex,
    targets: HashSet<VillagerIndex>,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum PlayerAction {
    TryReveal(VillagerIndex),
    TryExecute(VillagerIndex),
    Ability(AbilityAttempt),
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
