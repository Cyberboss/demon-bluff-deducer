use demon_bluff_gameplay_engine::game_state::{Action, GameState};
use demon_bluff_logic_engine::RevealStrategy;

mod game_set_1;
mod game_set_2;

fn run_game(game_state: &GameState, expected_actions: Vec<Action>, log_after: Option<usize>) {
	super::helpers::run_game(
		game_state,
		expected_actions,
		log_after,
		RevealStrategy::Simple,
	);
}
