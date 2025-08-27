mod breakpoint;
mod debug_break;
mod desire_node;
mod hypothesis_node;
mod node;
mod node_type;

use debug_break::DebugBreak;
use force_graph::ForceGraph;

pub use self::node::Node;

use super::FitnessAndAction;

pub struct Debugger {
    root_index: usize,
    nodes: Vec<Node>,
    graph: ForceGraph<usize, Option<FitnessAndAction>>,
    last_debug_break: Option<usize>,
}

impl Debugger {
    fn handle_break(debug_break: DebugBreak) {
        todo!()
    }
}
