use crate::engine::FitnessAndAction;

pub struct HypothesisNode {
    description: String,
    hypothesis_dependencies: Vec<usize>,
    desire_producer_dependencies: Vec<usize>,
    desire_consumer_dependencies: Vec<usize>,
    concluded: usize,
    current_fitness: Option<FitnessAndAction>,
}
