use itertools::Itertools;
use std::fmt::Display;

use crate::{
    Expression, testimony,
    villager::{self, VillagerArchetype, VillagerIndex},
};

pub const ALCHEMIST_CURE_RANGE: usize = 2;

#[derive(Clone, Debug, PartialEq, Eq, Display)]
pub enum ConfessorClaim {
    Good,
    Dizzy,
}

#[derive(Clone, Debug, PartialEq, Eq, Display, Hash)]
pub enum Direction {
    Clockwise,
    CounterClockwise,
    Equidistant,
}

#[derive(Clone, Debug, PartialEq, Eq, Display)]
pub enum ArchitectClaim {
    Left,
    Right,
    Equal,
}

#[derive(Clone, Debug, PartialEq, Eq, Display)]
pub enum BakerClaim {
    Original,
    Was(VillagerArchetype),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RoleClaim {
    villager: VillagerIndex,
    archetype: VillagerArchetype,
}

impl Display for RoleClaim {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} is {}", self.villager, self.archetype)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ScoutClaim {
    evil_role: VillagerArchetype,
    distance: u8,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EvilPairsClaim(u8);

impl Display for EvilPairsClaim {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} pairs of adjacent evils", self.0)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Display)]
pub enum Testimony {
    Good(VillagerIndex),
    Real(VillagerIndex),
    Evil(VillagerIndex),
    Corrupt(VillagerIndex),
    NotCorrupt(VillagerIndex),
    Lying(VillagerIndex),
    Cured(VillagerIndex),
    Architect(ArchitectClaim),
    Baker(BakerClaim),
    Role(RoleClaim),
    Enlightened(Direction),
    Invincible(VillagerIndex),
    Knitter(EvilPairsClaim),
    Affected(VillagerIndex),
    FakeEvil(VillagerIndex),
    SelfDestruct(VillagerIndex),
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

        let mut expr;
        if villagers_cured == 0 {
            expr = Expression::and_from_iterator(
                potential_indicies
                    .iter()
                    .map(|index| Testimony::NotCorrupt(VillagerIndex(*index))),
            );
        } else {
            expr = None;
            for combo in potential_indicies.iter().combinations(villagers_cured) {
                let new_expr = Expression::and_from_iterator(
                    combo
                        .iter()
                        .map(|index| Testimony::Cured(VillagerIndex(**index))),
                );

                let new_expr =
                    new_expr.expect("There should have been at least one villager that was cured");
                expr = Some(match expr {
                    Some(old_expr) => Expression::Or(Box::new(old_expr), Box::new(new_expr)),
                    None => new_expr,
                });
            }
        }

        expr.expect("logic error in cure expression builder")
    }
}
