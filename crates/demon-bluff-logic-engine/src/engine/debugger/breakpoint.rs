use std::{
	fmt::Display,
	sync::{Arc, Mutex},
	usize,
};

use super::context::DebuggerContext;
use crate::engine::{
	Cycle, DesireConsumerReference, DesireProducerReference, HypothesisReference, IndexReference,
};

pub enum Breakpoint {
	Initialize(Arc<Mutex<DebuggerContext>>),
	RegisterHypothesis(usize, bool),
	RegisterDesire(usize),
	IterationStart(usize),
	EnterHypothesis(usize),
	ExitHypothesis(usize),
	DesireUpdate(usize, usize, bool),
	DesireRead(usize),
	DetectCycle(Cycle),
	BreakCycle(Cycle, usize),
	CollapseDesire(usize),
}

impl Display for Breakpoint {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Initialize(_) => write!(f, "Initialize"),
			Self::RegisterHypothesis(index, root) => {
				write!(f, "Register {}", HypothesisReference::new(*index))?;
				if *root { write!(f, " (ROOT)") } else { Ok(()) }
			}
			Self::RegisterDesire(index) => {
				write!(f, "Register {}", DesireProducerReference::new(*index))
			}
			Self::IterationStart(iteration) => write!(f, "Start Iteration #{}", iteration),
			Self::EnterHypothesis(index) => {
				write!(f, "Enter {}", HypothesisReference::new(*index))
			}
			Self::ExitHypothesis(index) => {
				write!(f, "Exit {}", HypothesisReference::new(*index))
			}
			Self::DesireUpdate(hypothesis_index, desire_index, desired) => {
				write!(
					f,
					"Update {} from {}. Desired: {}",
					DesireProducerReference::new(*desire_index),
					HypothesisReference::new(*hypothesis_index),
					desired
				)
			}
			Self::DesireRead(index) => write!(f, "Read {}", DesireConsumerReference::new(*index)),
			Self::DetectCycle(cycle) => write!(f, "Dependency Cycle Encountered: {}", cycle),
			Self::BreakCycle(cycle, index) => write!(
				f,
				"Dependency Cycle broken at {}: {}",
				HypothesisReference::new(*index),
				cycle
			),
			Self::CollapseDesire(index) => {
				write!(f, "Collapse {}", DesireConsumerReference::new(*index))
			}
		}
	}
}
