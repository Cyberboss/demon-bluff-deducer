use std::fmt::Display;

use demon_bluff_gameplay_engine::villager::VillagerIndex;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct RevealVillagerDesire {
    index: VillagerIndex,
}

impl RevealVillagerDesire {
    pub fn new(index: VillagerIndex) -> Self {
        Self { index }
    }
}

impl Display for RevealVillagerDesire {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Reveal {}", self.index)
    }
}
