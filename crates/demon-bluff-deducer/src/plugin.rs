use bevy::{
	DefaultPlugins,
	app::{Plugin, Startup, Update},
	prelude::PluginGroup,
	state::app::AppExtStates,
	window::{Window, WindowPlugin},
};

use crate::{
	plugins::{evaluator::EvaluatorPlugin, menu::MenuPlugin},
	root_state::RootState,
	systems::{on_resize::on_resize, setup::setup},
};

pub struct RootPlugin;

impl Plugin for RootPlugin {
	fn build(&self, app: &mut bevy::app::App) {
		app.add_plugins(DefaultPlugins.set(WindowPlugin {
			primary_window: Some(Window {
				title: "Demon Bluff Deducer".to_string(),
				..Default::default()
			}),
			..Default::default()
		}))
		.init_state::<RootState>()
		.add_plugins(MenuPlugin)
		.add_plugins(EvaluatorPlugin)
		.add_systems(Startup, setup)
		.add_systems(Update, on_resize);
	}
}
