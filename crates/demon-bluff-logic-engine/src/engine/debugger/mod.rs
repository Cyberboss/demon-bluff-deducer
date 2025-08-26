mod breakpoint;
mod debug_break;
mod desire_node;
mod hypothesis_node;
mod node;
mod node_type;

pub use self::node::Node;

pub struct Debugger {
    root_index: usize,
    nodes: Vec<Node>,
}

impl Debugger {}
