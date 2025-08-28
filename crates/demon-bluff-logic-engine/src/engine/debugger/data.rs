use std::sync::{Arc, Mutex, MutexGuard};

use super::{Breakpoint, DebuggerContext, context::create_debugger_context};

#[derive(Debug, Clone)]
pub struct DebuggerData<FDebugBreak>
where
    FDebugBreak: FnMut(Breakpoint),
{
    context: Arc<Mutex<DebuggerContext>>,
    breaker: FDebugBreak,
}

impl<FDebugBreak> DebuggerData<FDebugBreak>
where
    FDebugBreak: FnMut(Breakpoint),
{
    pub fn new(mut breaker: FDebugBreak) -> Self {
        let context = Arc::new(Mutex::new(create_debugger_context()));
        breaker(Breakpoint::Initialize(context.clone()));
        Self { context, breaker }
    }

    pub fn context(&mut self) -> MutexGuard<DebuggerContext> {
        self.context.lock().expect("Debugger mutex was poisoned")
    }

    pub fn breaker(&mut self, breakpoint: Breakpoint) {
        (self.breaker)(breakpoint)
    }
}
