use crate::engine::{FitnessAndAction, HypothesisResult};

#[derive(Debug)]
pub struct HypothesisNode {
	description: String,
	hypothesis_dependencies: Vec<usize>,
	desire_producer_dependencies: Vec<usize>,
	desire_consumer_dependencies: Vec<usize>,
	concluded: bool,
	current_fitness: Option<FitnessAndAction>,
}

impl HypothesisNode {
	fn new(
		description: String,
		hypothesis_dependencies: Vec<usize>,
		desire_producer_dependencies: Vec<usize>,
		desire_consumer_dependencies: Vec<usize>,
	) -> Self {
		Self {
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

pub fn create_hypothesis_node(
	description: String,
	hypothesis_dependencies: Vec<usize>,
	desire_producer_dependencies: Vec<usize>,
	desire_consumer_dependencies: Vec<usize>,
) -> HypothesisNode {
	HypothesisNode::new(
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
