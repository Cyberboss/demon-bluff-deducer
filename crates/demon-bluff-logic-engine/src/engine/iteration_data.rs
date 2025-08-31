use super::{HypothesisResult, cycle::Cycle};

#[derive(Debug, PartialEq, Clone)]
pub enum VisitState {
	Unvisited,
	Visiting(HypothesisResult),
	Visited(HypothesisResult),
}

#[derive(Debug, PartialEq, Clone)]
pub struct IterationData {
	pub results: Vec<VisitState>,
}

#[derive(Debug)]
pub struct CurrentIterationData {
	pub inner: IterationData,
	/// these are cycles from the root, so they must be corrected upon revisit
	pub full_cycles: Vec<Vec<Cycle>>,
}
