use std::fmt::Display;

use crate::{Expression, testimony::Direction};

#[derive(Debug, Eq, PartialEq)]
pub struct VillagerAffect {
    direction: Direction,
    distance: u8,
}

#[derive(Debug, Eq, PartialEq, Display)]
pub enum NightEffect {
    KillUnrevealed,
}

#[derive(Debug, Eq, PartialEq, Display)]
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

impl Display for VillagerAffect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} villagers {}", self.distance, self.direction)
    }
}
