use std::process::{ExitCode, Termination};

use bevy::{prelude::*, render::view::RenderLayers};
use evaluator::EvaluatorPlugin;
use menu::MenuPlugin;
use state::RootState;

mod evaluator;
mod menu;
mod state;

fn main() -> ExitCode {
	println!("Hello, world!");

	App::new()
		.add_plugins(DefaultPlugins.set(WindowPlugin {
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
		.run()
		.report()
}

fn setup(mut commands: Commands, mut gizmos_config_store: ResMut<GizmoConfigStore>) {
	commands.spawn(Camera2d);

	let (default, _) = gizmos_config_store.config_mut::<DefaultGizmoConfigGroup>();
	// TODO: ensure gizmos are layered below other stuff
	default.render_layers = RenderLayers::from_layers(&[0]);
}
