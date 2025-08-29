use std::process::{ExitCode, Termination};

use bevy::app::App;
use plugin::RootPlugin;

mod plugin;
mod plugins;
mod root_state;
mod systems;

fn main() -> ExitCode {
	println!("Hello, world!");

	App::new().add_plugins(RootPlugin).run().report()
}
