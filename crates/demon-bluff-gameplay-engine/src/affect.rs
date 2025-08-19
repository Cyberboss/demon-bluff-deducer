use crate::{testimony::Direction, villager::Expression};

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
