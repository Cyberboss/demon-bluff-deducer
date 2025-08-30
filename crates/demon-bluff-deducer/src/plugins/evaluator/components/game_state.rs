use std::fs::File;

use bevy::prelude::*;
use demon_bluff_gameplay_engine::{
	game_state::{DrawStats, GameState, new_game},
	villager::{Demon, GoodVillager, Minion, Outcast, VillagerArchetype},
};

#[derive(Component)]
pub struct GameStateComponent {
	game_state: GameState,
}

impl GameStateComponent {
	pub fn new() -> Self {
		let file_reader = File::open("S:\\workspace\\demon-bluff-bot\\crates\\demon-bluff-logic-engine\\tests\\game_states\\gemcrafter_1_says_5_good.json").unwrap();
		let game_state: GameState = serde_json::from_reader(file_reader).unwrap();
		Self { game_state }
	}

	pub fn game_state(&self) -> &GameState {
		&self.game_state
	}
}
