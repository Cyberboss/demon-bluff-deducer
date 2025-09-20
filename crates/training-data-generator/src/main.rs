use std::process::{ExitCode, Termination};

use anyhow::Result;
use bevy::{
	DefaultPlugins,
	app::{App, Startup, Update},
};

use self::{
	data::image_id::ImageId,
	resources::window::WindowResource,
	systems::{
		keyboard_handler::keyboard_handler, mouse_button_handler::mouse_button_handler,
		setup::setup,
	},
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
		.add_systems(Update, (keyboard_handler, mouse_button_handler));

	let termination = app.run();

	Ok(termination.report())
}
