
use bevy::ecs::component::Component;
use demon_bluff_logic_engine::Breakpoint;

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
