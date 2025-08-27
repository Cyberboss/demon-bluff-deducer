use std::sync::{Arc, Mutex};

use crate::engine::{Cycle, FitnessAndAction};

use super::context::DebuggerContext;

pub enum Breakpoint {
    Initialize(Arc<Mutex<DebuggerContext>>),
    RegisterNode(usize),
    IterationStart(usize),
    EnterNode(usize),
    ExitNode(usize),
    DesireUpdate(usize),
    DetectCycle(Cycle),
    BreakCycle(Cycle),
    IterationEnd(usize),
}
