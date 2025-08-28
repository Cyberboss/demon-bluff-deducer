mod breakpoint;
mod context;
mod desire_node;
mod hypothesis_node;

use std::sync::{Arc, Mutex, MutexGuard};

use context::create_debugger_context;

pub use self::{
	breakpoint::Breakpoint,
	context::{DebuggerContext, desire_nodes_mut, hypothesis_nodes_mut},
	desire_node::{DesireNode, create_desire_node, update_desire_node},
	hypothesis_node::{HypothesisNode, create_hypothesis_node, update_hypothesis_node},
};

#[derive(Debug, Clone)]
pub struct Debugger<FDebugBreak>
where
	FDebugBreak: FnMut(Breakpoint),
{
	context: Arc<Mutex<DebuggerContext>>,
	breaker: FDebugBreak,
}

impl<FDebugBreak> Debugger<FDebugBreak>
where
	FDebugBreak: FnMut(Breakpoint),
{
	pub fn new(mut breaker: FDebugBreak) -> Self {
		let context = Arc::new(Mutex::new(create_debugger_context()));
		breaker(Breakpoint::Initialize(context.clone()));
		Self { context, breaker }
	}

	pub fn context<'a>(&'a mut self) -> MutexGuard<'a, DebuggerContext> {
		self.context.lock().expect("Debugger mutex was poisoned")
	}

	pub fn breakpoint(&mut self, breakpoint: Breakpoint) {
		(self.breaker)(breakpoint)
	}
}
