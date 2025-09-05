use std::collections::HashSet;

use demon_bluff_gameplay_engine::{
	game_state::GameState,
	villager::{Villager, VillagerIndex},
};

use crate::PlayerAction;

#[derive(Debug, Clone, Copy)]
pub enum RevealStrategy {
	Simple,
	FollowTestimony,
}

impl RevealStrategy {
	pub fn get_reveal(&self, log: &impl log::Log, game_state: &GameState) -> HashSet<PlayerAction> {
		match self {
			Self::Simple => simple_reveal(game_state),
			Self::FollowTestimony => follow_testimony_reveal(log, game_state),
		}
	}
}

fn simple_reveal(game_state: &GameState) -> HashSet<PlayerAction> {
	// Step three, need more info. Figure out how to best use reveals/abilities to gain info
	// For now just reveal the first hidden index and we'll make it better later
	let mut hidden_index = None;
	game_state.iter_villagers(|index, villager| {
		if hidden_index.is_none()
			&& let Villager::Hidden(_) = villager
		{
			hidden_index = Some(index);
			false
		} else {
			true
		}
	});

	let mut actions = HashSet::with_capacity(1);
	actions.insert(PlayerAction::TryReveal(
		hidden_index.expect("Reveal strategy was executed with no remaining villagers"),
	));
	actions
}

fn follow_testimony_reveal(log: &impl log::Log, game_state: &GameState) -> HashSet<PlayerAction> {
	todo!()
}
