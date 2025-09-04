use std::fmt::Display;

use crate::engine::{FitnessAndAction, HypothesisReference, HypothesisResult};

#[derive(Debug)]
pub struct HypothesisNode {
	reference: HypothesisReference,
	description: String,
	hypothesis_dependencies: Vec<usize>,
	desire_producer_dependencies: Vec<usize>,
	desire_consumer_dependencies: Vec<usize>,
	concluded: bool,
	current_fitness: Option<FitnessAndAction>,
}

impl HypothesisNode {
	fn new(
		reference: HypothesisReference,
		description: String,
		hypothesis_dependencies: Vec<usize>,
		desire_producer_dependencies: Vec<usize>,
		desire_consumer_dependencies: Vec<usize>,
	) -> Self {
		Self {
			reference,
			description,
			hypothesis_dependencies,
			desire_producer_dependencies,
			desire_consumer_dependencies,
			concluded: false,
			current_fitness: None,
		}
	}

	pub fn description(&self) -> &String {
		&self.description
	}

	pub fn hypothesis_dependencies(&self) -> &Vec<usize> {
		&self.hypothesis_dependencies
	}

	pub fn desire_producer_dependencies(&self) -> &Vec<usize> {
		&self.desire_producer_dependencies
	}

	pub fn desire_consumer_dependencies(&self) -> &Vec<usize> {
		&self.desire_consumer_dependencies
	}

	pub fn concluded(&self) -> bool {
		self.concluded
	}

	pub fn current_fitness(&self) -> Option<&FitnessAndAction> {
		self.current_fitness.as_ref()
	}
}

impl Display for HypothesisNode {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}: {}", self.reference, self.description)
	}
}

pub fn create_hypothesis_node(
	reference: HypothesisReference,
	description: String,
	hypothesis_dependencies: Vec<usize>,
	desire_producer_dependencies: Vec<usize>,
	desire_consumer_dependencies: Vec<usize>,
) -> HypothesisNode {
	HypothesisNode::new(
		reference,
		description,
		hypothesis_dependencies,
		desire_producer_dependencies,
		desire_consumer_dependencies,
	)
}

pub fn update_hypothesis_node(node: &mut HypothesisNode, result: &HypothesisResult) {
	match result {
		HypothesisResult::Pending(fitness_and_action) => {
			node.current_fitness = Some(fitness_and_action.clone())
		}
		HypothesisResult::Conclusive(fitness_and_action) => {
			node.current_fitness = Some(fitness_and_action.clone());
			node.concluded = true
		}
	}
}
