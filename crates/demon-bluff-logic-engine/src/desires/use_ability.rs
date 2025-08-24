use std::fmt::Display;

use demon_bluff_gameplay_engine::villager::VillagerIndex;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct UseAbilityDesire {
    index: VillagerIndex,
}

impl UseAbilityDesire {
    pub fn new(index: VillagerIndex) -> Self {
        Self { index }
    }
}

impl Display for UseAbilityDesire {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Use {}'s ability", self.index)
    }
}
