use std::process::{ExitCode, Termination};

use anyhow::Result;
use bevy::{
	DefaultPlugins,
	app::{App, Startup, Update},
};

use self::{
	resources::window::WindowResource,
	systems::{
		keyboard_handler::keyboard_handler, mouse_button_handler::mouse_button_handler,
		setup::setup, text_input::text_input, word_selection_renderer::word_selection_renderer,
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
		.add_systems(
			Update,
			(
				keyboard_handler,
				mouse_button_handler,
				word_selection_renderer,
				text_input,
			),
		);

	let termination = app.run();

	Ok(termination.report())
}
