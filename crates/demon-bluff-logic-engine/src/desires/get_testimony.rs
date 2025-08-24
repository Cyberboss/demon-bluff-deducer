use std::fmt::Display;

use demon_bluff_gameplay_engine::villager::VillagerIndex;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct GetTestimonyDesire {
    index: VillagerIndex,
}

impl GetTestimonyDesire {
    pub fn new(index: VillagerIndex) -> Self {
        Self { index }
    }
}

impl Display for GetTestimonyDesire {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Get {}'s Testimony", self.index)
    }
}
