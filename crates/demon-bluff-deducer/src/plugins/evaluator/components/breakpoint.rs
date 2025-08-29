use std::{
	collections::HashSet,
	sync::{Arc, Mutex},
};

use bevy::{ecs::component::Component, tasks::Task};
use demon_bluff_logic_engine::{Breakpoint, DebuggerContext, PlayerAction, PredictionError};

#[derive(Component)]
pub struct BreakpointComponent {
	breakpoint: Breakpoint,
}

impl BreakpointComponent {
	pub fn new(breakpoint: Breakpoint) -> Self {
		Self { breakpoint }
	}

	pub fn breakpoint(&self) -> &Breakpoint {
		&self.breakpoint
	}
}
