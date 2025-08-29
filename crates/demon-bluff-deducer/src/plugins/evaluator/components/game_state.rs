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
		Self {
			game_state: new_game(
				vec![
					VillagerArchetype::GoodVillager(GoodVillager::Druid),
					VillagerArchetype::GoodVillager(GoodVillager::Architect),
					VillagerArchetype::GoodVillager(GoodVillager::Medium),
					VillagerArchetype::GoodVillager(GoodVillager::Gemcrafter),
					VillagerArchetype::GoodVillager(GoodVillager::Slayer),
					VillagerArchetype::GoodVillager(GoodVillager::Alchemist),
					VillagerArchetype::Outcast(Outcast::Bombardier),
					VillagerArchetype::Minion(Minion::Witch),
					VillagerArchetype::Demon(Demon::Pooka),
				],
				DrawStats::new(4, 1, 1, 1),
				2,
				false,
			),
		}
	}

	pub fn game_state(&self) -> &GameState {
		&self.game_state
	}
}
