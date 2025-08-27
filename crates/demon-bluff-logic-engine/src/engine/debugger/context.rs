use super::node::Node;

pub struct DebuggerContext {
    nodes: Vec<Node>,
}

impl DebuggerContext {
    pub fn new() -> Self {
        Self { nodes: Vec::new() }
    }

    pub fn nodes(&self) -> &Vec<Node> {
        &self.nodes
    }
}

pub fn create_debugger_context() -> DebuggerContext {
    DebuggerContext::new()
}
