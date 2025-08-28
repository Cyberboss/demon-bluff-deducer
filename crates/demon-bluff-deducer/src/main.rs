use bevy::prelude::*;
use menu::MenuPlugin;
use root_state::RootState;

mod evaluator;
mod menu;
mod root_state;

fn main() {
	println!("Hello, world!");

	App::new()
		.add_plugins(DefaultPlugins)
		.init_state::<RootState>()
		.add_plugins(MenuPlugin)
		.add_systems(Startup, setup)
		.run();
}

fn setup(mut commands: Commands) {
	commands.spawn(Camera2d);
}
