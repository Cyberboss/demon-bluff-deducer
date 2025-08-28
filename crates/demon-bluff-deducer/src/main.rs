use std::process::{ExitCode, Termination};

use bevy::prelude::*;
use evaluator::EvaluatorPlugin;
use menu::MenuPlugin;
use state::RootState;

mod evaluator;
mod menu;
mod state;

fn main() -> ExitCode {
	println!("Hello, world!");

	App::new()
		.add_plugins(DefaultPlugins)
		.init_state::<RootState>()
		.add_plugins(MenuPlugin)
		.add_plugins(EvaluatorPlugin)
		.add_systems(Startup, setup)
		.run()
		.report()
}

fn setup(mut commands: Commands) {
	commands.spawn(Camera2d);
}
