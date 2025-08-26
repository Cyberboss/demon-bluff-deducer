use crate::engine::{Cycle, debugger::node::Node};

pub enum Breakpoint<'a> {
    RegisterNode(usize),
    IterationStart(usize),
    EnterNode(usize),
    DesireUpdate(usize),
    DetectCycle(&'a Cycle),
    BreakCycle(&'a Cycle),
    IterationEnd(usize),
}
