use crate::{Expression, testimony::Direction};

pub struct VillagerAffect {
    direction: Direction,
    distance: u8,
}

pub enum NightEffect {
    KillUnrevealed,
}

pub enum Affect {
    Corrupt(Expression<VillagerAffect>),
    Puppet(Expression<VillagerAffect>),
    Night(NightEffect),
    FakeOutcast,
    DupeVillager,
}

impl VillagerAffect {
    pub fn new(direction: Direction, distance: u8) -> Self {
        Self {
            direction,
            distance,
        }
    }
}
