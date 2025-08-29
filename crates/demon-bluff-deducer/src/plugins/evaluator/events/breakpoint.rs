use bevy::prelude::*;
use demon_bluff_logic_engine::Breakpoint;

#[derive(Event)]
pub struct BreakpointEvent {
	breakpoint: Breakpoint,
}

impl BreakpointEvent {
	pub fn new(breakpoint: Breakpoint) -> Self {
		Self { breakpoint }
	}

	pub fn breakpoint(&self) -> &Breakpoint {
		&self.breakpoint
	}
}
