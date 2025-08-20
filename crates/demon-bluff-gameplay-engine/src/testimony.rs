use itertools::Itertools;

use crate::{
    Expression, testimony,
    villager::{self, VillagerArchetype, VillagerIndex},
};

pub const ALCHEMIST_CURE_RANGE: usize = 2;

#[derive(Clone, Debug)]
pub enum ConfessorClaim {
    Good,
    Dizzy,
}

#[derive(Clone, Debug)]
pub enum Direction {
    Clockwise,
    CounterClockwise,
}

#[derive(Clone, Debug)]
pub enum ArchitectClaim {
    Left,
    Right,
    Equal,
}

#[derive(Clone, Debug)]
pub enum EnlightendClaim {
    Equidistant,
    Direction(Direction),
}

#[derive(Clone, Debug)]
pub enum BakerClaim {
    Original,
    Was(VillagerArchetype),
}

#[derive(Clone, Debug)]
pub struct RoleClaim {
    villager: VillagerIndex,
    archetype: VillagerArchetype,
}

#[derive(Clone, Debug)]
pub struct ScoutClaim {
    evil_role: VillagerArchetype,
    distance: u8,
}

#[derive(Clone, Debug)]
pub struct EvilPairsClaim(u8);

#[derive(Clone, Debug)]
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
    Slayed(VillagerIndex),
    Confess(ConfessorClaim),
}

impl EvilPairsClaim {
    pub fn new(pair_count: u8) -> Self {
        Self(pair_count)
    }
}

impl RoleClaim {
    pub fn new(villager: VillagerIndex, archetype: VillagerArchetype) -> Self {
        Self {
            villager,
            archetype,
        }
    }
}

impl Testimony {
    pub fn cure(
        start_index: VillagerIndex,
        villagers_cured: usize,
        total_villagers: usize,
        cure_range: usize,
    ) -> Expression<Testimony> {
        if villagers_cured == 0 {
            return Expression::Unary(Testimony::Cured(Vec::<VillagerIndex>::new()));
        }

        let start_index = start_index.0;
        let candidate_count = cure_range * 2;
        let mut potential_indicies = Vec::with_capacity(candidate_count);
        let mut current_index = if start_index >= cure_range {
            start_index - cure_range
        } else {
            total_villagers - (cure_range - start_index)
        };

        for _ in 0..cure_range {
            potential_indicies.push(current_index);
            current_index = if current_index + 1 >= total_villagers {
                0
            } else {
                current_index + 1
            }
        }

        current_index = if current_index + 1 >= total_villagers {
            0
        } else {
            current_index + 1
        };

        for _ in 0..cure_range {
            potential_indicies.push(current_index);
            current_index = if current_index + 1 >= total_villagers {
                0
            } else {
                current_index + 1
            }
        }

        let mut expr = None;
        for combo in potential_indicies.iter().combinations(villagers_cured) {
            let new_expr = Expression::Unary(Testimony::Cured(
                combo.iter().map(|index| VillagerIndex(**index)).collect(),
            ));
            expr = Some(match expr {
                Some(old_expr) => Expression::Or(Box::new(old_expr), Box::new(new_expr)),
                None => new_expr,
            });
        }

        return expr.expect("logic error in cure expression builder");
    }
}
