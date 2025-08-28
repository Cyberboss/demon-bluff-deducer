use std::fmt::{Display, Formatter};

use serde::Serialize;

use crate::engine::fitness_and_action::FitnessAndAction;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub enum HypothesisResult {
    Pending(FitnessAndAction),
    Conclusive(FitnessAndAction),
}

impl HypothesisResult {
    pub fn unimplemented() -> Self {
        Self::Conclusive(FitnessAndAction::unimplemented())
    }

    pub fn impossible() -> Self {
        Self::Conclusive(FitnessAndAction::impossible())
    }

    pub fn map<F>(self, mut f: F) -> Self
    where
        F: FnMut(FitnessAndAction) -> FitnessAndAction,
    {
        match self {
            Self::Pending(fitness_and_action) => Self::Pending(f(fitness_and_action)),
            Self::Conclusive(fitness_and_action) => Self::Conclusive(f(fitness_and_action)),
        }
    }

    pub fn fitness_and_action(&self) -> &FitnessAndAction {
        match self {
            HypothesisResult::Pending(fitness_and_action)
            | HypothesisResult::Conclusive(fitness_and_action) => fitness_and_action,
        }
    }
}

impl Display for HypothesisResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            HypothesisResult::Pending(fitness_and_action) => {
                write!(f, "Pending: {fitness_and_action}")
            }
            HypothesisResult::Conclusive(fitness_and_action) => {
                write!(f, "Conclusive: {fitness_and_action}")
            }
        }
    }
}
