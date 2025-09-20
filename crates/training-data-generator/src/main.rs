use std::process::{ExitCode, Termination};

use anyhow::Result;
use bevy::{
	DefaultPlugins,
	app::{App, Startup, Update},
};
use systems::setup::setup;

use self::{
	data::image_id::ImageId, resources::window::WindowResource,
	systems::keyboard_handler::keyboard_handler,
};

mod components;
mod data;
mod resources;
mod systems;

fn main() -> Result<ExitCode> {
	let mut app = App::new();

	app.add_plugins(DefaultPlugins)
		.insert_resource(WindowResource::new()?)
		.add_systems(Startup, setup)
		.add_systems(Update, keyboard_handler);

	let termination = app.run();

	Ok(termination.report())
}
