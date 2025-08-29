use std::{
	collections::HashSet,
	fmt::{Display, Formatter},
};

use serde::Serialize;

use super::HypothesisResult;
use crate::player_action::PlayerAction;

pub const FITNESS_UNKNOWN: f64 = 0.5;
pub const FITNESS_UNIMPLEMENTED: f64 = 0.000123456789;

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

	pub fn unknown(action: Option<PlayerAction>) -> Self {
		Self::new(FITNESS_UNKNOWN, action)
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

				write!(f, "[{action}]")?
			}
		}

		Ok(())
	}
}

pub fn decide_result(lhs: HypothesisResult, rhs: HypothesisResult) -> HypothesisResult {
	if lhs.fitness_and_action().fitness > rhs.fitness_and_action().fitness {
		lhs
	} else {
		rhs
	}
}

pub fn and_result(lhs: HypothesisResult, rhs: HypothesisResult) -> HypothesisResult {
	let new_fitness_and_action;
	let must_be_pending;
	match lhs {
		HypothesisResult::Pending(fitness_and_action) => {
			must_be_pending = true;
			new_fitness_and_action = fitness_and_action
		}
		HypothesisResult::Conclusive(fitness_and_action) => {
			must_be_pending = false;
			new_fitness_and_action = fitness_and_action
		}
	}
	match rhs {
		HypothesisResult::Pending(current_fitness_and_action) => HypothesisResult::Pending(
			and_fitness(current_fitness_and_action, new_fitness_and_action),
		),
		HypothesisResult::Conclusive(current_fitness_and_action) => {
			let merged = and_fitness(current_fitness_and_action, new_fitness_and_action);

			if must_be_pending {
				HypothesisResult::Pending(merged)
			} else {
				HypothesisResult::Conclusive(merged)
			}
		}
	}
}

pub fn or_result(lhs: HypothesisResult, rhs: HypothesisResult) -> HypothesisResult {
	let new_fitness_and_action;
	let must_be_pending;
	match lhs {
		HypothesisResult::Pending(fitness_and_action) => {
			must_be_pending = true;
			new_fitness_and_action = fitness_and_action
		}
		HypothesisResult::Conclusive(fitness_and_action) => {
			must_be_pending = false;
			new_fitness_and_action = fitness_and_action
		}
	}
	match rhs {
		HypothesisResult::Pending(current_fitness_and_action) => HypothesisResult::Pending(
			or_fitness(current_fitness_and_action, new_fitness_and_action),
		),
		HypothesisResult::Conclusive(current_fitness_and_action) => {
			let merged = or_fitness(current_fitness_and_action, new_fitness_and_action);

			if must_be_pending {
				HypothesisResult::Pending(merged)
			} else {
				HypothesisResult::Conclusive(merged)
			}
		}
	}
}

pub fn average_result(results: impl Iterator<Item = HypothesisResult>) -> Option<HypothesisResult> {
	let mut fitness_sum = 0.0;
	let mut total_items: usize = 0;
	let mut pending = false;

	for result in results {
		let fitness = match result {
			HypothesisResult::Pending(fitness_and_action) => {
				pending = true;
				fitness_and_action
			}
			HypothesisResult::Conclusive(fitness_and_action) => fitness_and_action,
		};

		fitness_sum += fitness.fitness;
		total_items += 1;
	}

	if total_items == 0 {
		None
	} else {
		let average_fitness = fitness_sum / (total_items as f64);
		Some(if pending {
			HypothesisResult::Pending(FitnessAndAction::new(average_fitness, None))
		} else {
			HypothesisResult::Conclusive(FitnessAndAction::new(average_fitness, None))
		})
	}
}

pub fn fittest_result(lhs: HypothesisResult, rhs: HypothesisResult) -> HypothesisResult {
	let new_fitness_and_action;
	let must_be_pending;
	match lhs {
		HypothesisResult::Pending(fitness_and_action) => {
			must_be_pending = true;
			new_fitness_and_action = fitness_and_action
		}
		HypothesisResult::Conclusive(fitness_and_action) => {
			must_be_pending = false;
			new_fitness_and_action = fitness_and_action
		}
	}
	match rhs {
		HypothesisResult::Pending(current_fitness_and_action) => HypothesisResult::Pending(
			if current_fitness_and_action.fitness > new_fitness_and_action.fitness {
				current_fitness_and_action
			} else {
				new_fitness_and_action
			},
		),
		HypothesisResult::Conclusive(current_fitness_and_action) => {
			let fittest = if current_fitness_and_action.fitness > new_fitness_and_action.fitness {
				current_fitness_and_action
			} else {
				new_fitness_and_action
			};

			if must_be_pending {
				HypothesisResult::Pending(fittest)
			} else {
				HypothesisResult::Conclusive(fittest)
			}
		}
	}
}

pub fn and_fitness(mut lhs: FitnessAndAction, rhs: FitnessAndAction) -> FitnessAndAction {
	for rh_action in rhs.action {
		lhs.action.insert(rh_action);
	}

	// P(A and B) = P(A) * P(B)
	lhs.fitness *= rhs.fitness;
	lhs
}

pub fn or_fitness(mut lhs: FitnessAndAction, rhs: FitnessAndAction) -> FitnessAndAction {
	for rh_action in rhs.action {
		lhs.action.insert(rh_action);
	}

	// P(A or B) = P(A) + P(B) - P(A and B)
	lhs.fitness = lhs.fitness + rhs.fitness - (lhs.fitness * rhs.fitness);
	lhs
}
