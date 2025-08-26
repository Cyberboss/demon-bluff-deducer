use log::Log;

use crate::{
    engine::{debugger::Debugger, stack_data::StackData},
    hypotheses::{DesireType, HypothesisType},
};

pub struct DebugBreak<'a, TLog>
where
    TLog: Log,
{
    stack_data: Option<StackData<'a, TLog, HypothesisType, DesireType>>,
}

impl<'a, TLog> DebugBreak<'a, TLog>
where
    TLog: Log,
{
    pub fn into_debugger() -> Debugger {
        todo!()
    }
}
