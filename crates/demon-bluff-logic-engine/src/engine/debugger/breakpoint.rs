use std::sync::{Arc, Mutex};

use super::context::DebuggerContext;
use crate::engine::Cycle;

pub enum Breakpoint {
	Initialize(Arc<Mutex<DebuggerContext>>),
	RegisterNode(usize),
	IterationStart(usize),
	EnterNode(usize),
	ExitNode(usize),
	DesireUpdate(usize),
	DetectCycle(Cycle),
	BreakCycle(Cycle),
}
