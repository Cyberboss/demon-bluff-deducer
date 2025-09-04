use super::{DesireNode, HypothesisNode};

#[derive(Debug)]
pub struct DebuggerContext {
	hypotheses: Vec<HypothesisNode>,
	desires: Vec<DesireNode>,
}

impl DebuggerContext {
	fn new() -> Self {
		Self {
			hypotheses: Vec::new(),
			desires: Vec::new(),
		}
	}

	pub fn hypotheses(&self) -> &Vec<HypothesisNode> {
		&self.hypotheses
	}

	pub fn desires(&self) -> &Vec<DesireNode> {
		&self.desires
	}
}

pub fn create_debugger_context() -> DebuggerContext {
	DebuggerContext::new()
}

pub fn hypothesis_nodes_mut(context: &mut DebuggerContext) -> &mut Vec<HypothesisNode> {
	&mut context.hypotheses
}

pub fn desire_nodes_mut(context: &mut DebuggerContext) -> &mut Vec<DesireNode> {
	&mut context.desires
}
