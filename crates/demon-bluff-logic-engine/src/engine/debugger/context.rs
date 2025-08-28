use super::node::Node;

pub struct DebuggerContext {
    nodes: Vec<Node>,
}

impl DebuggerContext {
    fn new() -> Self {
        Self { nodes: Vec::new() }
    }

    pub fn nodes(&self) -> &Vec<Node> {
        &self.nodes
    }
}

pub fn create_debugger_context() -> DebuggerContext {
    DebuggerContext::new()
}

pub fn nodes_mut(debugger: &mut DebuggerContext) -> &mut Vec<Node> {
    &mut debugger.nodes
}
