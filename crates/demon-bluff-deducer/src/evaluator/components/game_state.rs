use bevy::prelude::*;
use demon_bluff_gameplay_engine::game_state::GameState;

#[derive(Component)]
pub struct GameStateComponent {
	game_state: GameState,
}
