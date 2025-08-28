use bevy::prelude::*;
use demon_bluff_gameplay_engine::{
	game_state::{DrawStats, new_game},
	villager::{Demon, GoodVillager, Minion, Outcast, VillagerArchetype},
};

use crate::{
	evaluator::GameStateComponent, menu::components::button_action::MenuButtonAction,
	state::RootState,
};

pub fn menu_action(
	interaction_query: Query<
		(&Interaction, &MenuButtonAction),
		(Changed<Interaction>, With<Button>),
	>,
	mut commands: Commands,
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
					commands.spawn(GameStateComponent::new());
					game_state.set(RootState::Evaluation);
				}
			}
		}
	}
}
