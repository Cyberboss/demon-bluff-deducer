use std::{
    collections::HashSet,
    fmt::{Display, Formatter},
};

use serde::Serialize;

use crate::player_action::PlayerAction;

pub const FITNESS_UNKNOWN: f64 = 0.5;
const FITNESS_UNIMPLEMENTED: f64 = 0.000123456789;

/// Contains the fitness score of a given action set.
/// Fitness is the probability of how much a given `PlayerAction` will move the `GameState` towards a winning conclusion.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct FitnessAndAction {
    action: HashSet<PlayerAction>,
    fitness: f64,
}

impl FitnessAndAction {
    pub fn new(fitness: f64, action: Option<PlayerAction>) -> Self {
        let mut action_set = HashSet::with_capacity(1);
        if let Some(action) = action {
            action_set.insert(action);
        }

        Self {
            action: action_set,
            fitness,
        }
    }

    pub fn invert(mut self) -> Self {
        self.fitness = 1.0 - self.fitness;
        self
    }

    pub fn impossible() -> Self {
        Self {
            action: HashSet::new(),
            fitness: 0.0,
        }
    }

    pub fn unimplemented() -> Self {
        Self {
            action: HashSet::new(),
            fitness: FITNESS_UNIMPLEMENTED,
        }
    }

    pub fn certainty(action: Option<PlayerAction>) -> Self {
        Self::new(1.0, action)
    }

    pub fn is_certain(&self) -> bool {
        self.fitness == 1.0
    }

    pub fn fitness(&self) -> f64 {
        self.fitness
    }

    pub fn action(&self) -> &HashSet<PlayerAction> {
        &self.action
    }
}

impl Display for FitnessAndAction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.fitness == 0.0 {
            return write!(f, "Impossible");
        }

        if self.fitness == FITNESS_UNIMPLEMENTED {
            return write!(f, "UNIMPLEMENTED");
        }

        write!(f, "{:.2}%", self.fitness * 100.0)?;

        let length = self.action.len();
        if length > 0 {
            write!(f, " - ")?;

            for (index, action) in self.action.iter().enumerate() {
                if index != 0 {
                    write!(f, ", ")?
                } else if index == length {
                    write!(f, "or ")?
                }

                write!(f, "[{}]", action)?
            }
        }

        Ok(())
    }
}
