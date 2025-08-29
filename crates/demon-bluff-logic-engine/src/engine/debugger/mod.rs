mod breakpoint;
mod context;
mod desire_node;
mod hypothesis_node;

use std::sync::{Arc, RwLock, RwLockWriteGuard};

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
	context: Arc<RwLock<DebuggerContext>>,
	breaker: FDebugBreak,
}

impl<FDebugBreak> Debugger<FDebugBreak>
where
	FDebugBreak: FnMut(Breakpoint),
{
	pub fn new(mut breaker: FDebugBreak) -> Self {
		let context = Arc::new(RwLock::new(create_debugger_context()));
		breaker(Breakpoint::Initialize(context.clone()));
		Self { context, breaker }
	}

	pub fn context<'a>(&'a mut self) -> RwLockWriteGuard<'a, DebuggerContext> {
		self.context.write().expect("Debugger mutex was poisoned")
	}

	pub fn breakpoint(&mut self, breakpoint: Breakpoint) {
		(self.breaker)(breakpoint)
	}
}
