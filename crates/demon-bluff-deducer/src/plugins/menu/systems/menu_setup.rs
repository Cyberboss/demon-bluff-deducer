use bevy::prelude::*;

use crate::plugins::menu::state::MenuState;

pub fn menu_setup(mut menu_state: ResMut<NextState<MenuState>>) {
	menu_state.set(MenuState::Main);
}
