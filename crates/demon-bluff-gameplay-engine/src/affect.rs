use std::{fmt::Display, num::NonZeroU8};

use crate::{Expression, testimony::Direction, villager::VillagerIndex};

#[derive(Debug, Eq, PartialEq)]
pub struct VillagerAffect {
    direction: Direction,
    distance: u8,
}

#[derive(Debug, Eq, PartialEq, Display)]
pub enum NightEffect {
    KillUnrevealed,
}

/// An Affect is a permanant state change a [`crate::villager::VillagerArchetype`] does without testifying it
#[derive(Debug, Eq, PartialEq, Display)]
pub enum Affect {
    Corrupt(Expression<VillagerAffect>),
    Puppet(Expression<VillagerAffect>),
    Night(NightEffect),
    DupeVillager,
    FakeOutcast,
    BlockLastNReveals(u8),
}

impl VillagerAffect {
    pub fn new(direction: Direction, distance: u8) -> Self {
        Self {
            direction,
            distance,
        }
    }

    pub fn from_index(
        source: &VillagerIndex,
        target: &VillagerIndex,
        total_villagers: usize,
    ) -> Self {
        let mut left_distance = 0;
        let mut test_index = source.0;
        loop {
            if test_index == target.0 {
                break;
            }

            test_index = test_index + 1;
            left_distance = left_distance + 1;

            if test_index == total_villagers {
                test_index = 0;
            }
        }
        let mut right_distance = 0;

        test_index = source.0;
        loop {
            if test_index == target.0 {
                break;
            }

            right_distance = right_distance + 1;

            if test_index == 0 {
                test_index = total_villagers - 1;
            } else {
                test_index = test_index - 1;
            }
        }

        let direction;
        let distance;
        if left_distance < right_distance {
            direction = Direction::Clockwise;
            distance = left_distance;
        } else {
            distance = right_distance;
            direction = if right_distance < left_distance {
                Direction::CounterClockwise
            } else {
                Direction::Equidistant
            }
        }

        Self::new(direction, distance)
    }
}

impl Display for VillagerAffect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} villagers {}", self.distance, self.direction)
    }
}
