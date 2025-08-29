use bevy::prelude::*;

use super::{
	components::on_main_menu_screen::OnMainMenuScreen,
	state::MenuState,
	systems::{
		button::button, despawn_screen, main_menu_setup::main_menu_setup, menu_action::menu_action,
		menu_setup::menu_setup,
	},
};
use crate::root_state::RootState;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
	fn build(&self, app: &mut App) {
		app
			// At start, the menu is not enabled. This will be changed in `menu_setup` when
			// entering the `GameState::Menu` state.
			// Current screen in the menu is handled by an independent state from `GameState`
			.add_sub_state::<MenuState>()
			.add_systems(OnEnter(RootState::Menu), menu_setup)
			// Systems to handle the main menu screen
			.add_systems(OnEnter(MenuState::Main), main_menu_setup)
			.add_systems(
				OnExit(MenuState::Main),
				despawn_screen::despawn_screen::<OnMainMenuScreen>,
			)
			.add_systems(
				Update,
				(
					button.run_if(in_state(RootState::Menu)),
					menu_action.run_if(in_state(MenuState::Main)),
				),
			);
	}
}
