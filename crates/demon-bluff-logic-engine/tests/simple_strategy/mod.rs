use demon_bluff_gameplay_engine::game_state::{Action, GameState};
use demon_bluff_logic_engine::RevealStrategy;

use crate::helpers::TestAction;

mod game_set_1;
mod game_set_2;
mod game_set_3;

fn run_game(game_state: &GameState, expected_actions: Vec<TestAction>, log_after: Option<usize>) {
	super::helpers::run_game(
		game_state,
		expected_actions,
		log_after,
		RevealStrategy::Simple,
	);
}

fn run_game_ack_unsolvable(
	game_state: &GameState,
	expected_actions: Vec<TestAction>,
	log_after: Option<usize>,
) {
	super::helpers::run_game_ack_unsolvable(
		game_state,
		expected_actions,
		log_after,
		RevealStrategy::Simple,
	);
}
