use crate::villager::{VillagerArchetype, VillagerIndex};

pub enum ConfessorClaim {
    Good,
    Dizzy,
}

pub enum Direction {
    Clockwise,
    CounterClockwise,
}

pub enum ArchitectClaim {
    Left,
    Right,
    Equal,
}

pub enum EnlightendClaim {
    Equidistant,
    Direction(Direction),
}

pub enum BakerClaim {
    Original,
    Was(VillagerArchetype),
}

pub struct RoleClaim {
    villager: VillagerIndex,
    evil_role: VillagerArchetype,
}

pub struct ScoutClaim {
    evil_role: VillagerArchetype,
    distance: u8,
}

pub struct EvilPairs(u8);

pub enum Testimony {
    Good(Vec<VillagerIndex>),
    Real(Vec<VillagerIndex>),
    Evil(Vec<VillagerIndex>),
    Corrupt(Vec<VillagerIndex>),
    Lying(Vec<VillagerIndex>),
    Cured(Vec<VillagerIndex>),
    Architect(ArchitectClaim),
    Baker(BakerClaim),
    Role(Vec<RoleClaim>),
    Enlightened(EnlightendClaim),
    Invincible(Vec<VillagerIndex>),
    Knitter(EvilPairs),
    Affected(Vec<VillagerIndex>),
    FakeEvil(Vec<VillagerIndex>),
    SelfDestruct(Vec<VillagerIndex>),
}

impl EvilPairs {
    pub fn new(pair_count: u8) -> Self {
        Self(pair_count)
    }
}
