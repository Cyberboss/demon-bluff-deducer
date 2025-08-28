use bevy::prelude::*;

use crate::{menu::components::button_action::MenuButtonAction, root_state::RootState};

pub fn menu_action(
	interaction_query: Query<
		(&Interaction, &MenuButtonAction),
		(Changed<Interaction>, With<Button>),
	>,
	mut app_exit_events: EventWriter<AppExit>,
	mut game_state: ResMut<NextState<RootState>>,
) {
	for (interaction, menu_button_action) in &interaction_query {
		if *interaction == Interaction::Pressed {
			match menu_button_action {
				MenuButtonAction::Exit => {
					app_exit_events.write(AppExit::Success);
				}
				MenuButtonAction::DoThing => {
					game_state.set(RootState::Evaluation);
				}
			}
		}
	}
}
