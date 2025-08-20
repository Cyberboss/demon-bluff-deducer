use crate::villager::{VillagerArchetype, VillagerIndex};

pub enum ConfessorClaim {
    Good,
    Dizzy,
}

#[derive(Clone)]
pub enum Direction {
    Clockwise,
    CounterClockwise,
}

#[derive(Clone)]
pub enum ArchitectClaim {
    Left,
    Right,
    Equal,
}

#[derive(Clone)]
pub enum EnlightendClaim {
    Equidistant,
    Direction(Direction),
}

#[derive(Clone)]
pub enum BakerClaim {
    Original,
    Was(VillagerArchetype),
}

#[derive(Clone)]
pub struct RoleClaim {
    villager: VillagerIndex,
    evil_role: VillagerArchetype,
}

#[derive(Clone)]
pub struct ScoutClaim {
    evil_role: VillagerArchetype,
    distance: u8,
}

#[derive(Clone)]
pub struct EvilPairsClaim(u8);

#[derive(Clone)]
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
    Knitter(EvilPairsClaim),
    Affected(Vec<VillagerIndex>),
    FakeEvil(Vec<VillagerIndex>),
    SelfDestruct(Vec<VillagerIndex>),
}

impl EvilPairsClaim {
    pub fn new(pair_count: u8) -> Self {
        Self(pair_count)
    }
}
