use std::{
	fmt::{Display, write},
	sync::{Arc, Mutex},
	usize,
};

use super::context::DebuggerContext;
use crate::engine::Cycle;

pub enum Breakpoint {
	Initialize(Arc<Mutex<DebuggerContext>>),
	RegisterHypothesis(usize),
	RegisterDesire(usize),
	IterationStart(usize),
	EnterHypothesis(usize),
	ExitHypothesis(usize),
	DesireUpdate(usize),
	DesireRead(usize),
	DetectCycle(Cycle),
	BreakCycle(Cycle, usize),
	CollapseDesire(usize),
}

impl Display for Breakpoint {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Initialize(_) => write!(f, "Initialize"),
			Self::RegisterHypothesis(index) => write!(f, "Register Hypothesis #{}", index + 1),
			Self::RegisterDesire(index) => write!(f, "Register Desire #{}", index + 1),
			Self::IterationStart(iteration) => write!(f, "Start Iteration #{}", iteration),
			Self::EnterHypothesis(index) => write!(f, "Enter Hypothesis #{}", index + 1),
			Self::ExitHypothesis(index) => write!(f, "Exit Hypothesis #{}", index + 1),
			Self::DesireUpdate(index) => write!(f, "Update Desire #{}", index + 1),
			Self::DesireRead(index) => write!(f, "Read Desire #{}", index + 1),
			Self::DetectCycle(_) => write!(f, "Dependency Cycle Encountered"),
			Self::BreakCycle(_, index) => {
				write!(f, "Dependency Cycle Broken at Hypothesis #{}", index + 1)
			}
			Self::CollapseDesire(index) => write!(f, "Collapse Desire #{}", index + 1),
		}
	}
}
