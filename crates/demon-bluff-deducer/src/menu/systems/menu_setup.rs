use bevy::prelude::*;

use crate::menu::state::MenuState;

pub fn menu_setup(mut menu_state: ResMut<NextState<MenuState>>) {
    menu_state.set(MenuState::Main);
}
