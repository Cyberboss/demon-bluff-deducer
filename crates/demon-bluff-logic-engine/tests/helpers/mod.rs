use std::{
	fs::{self, File},
	path::PathBuf,
};

use demon_bluff_gameplay_engine::game_state::{Action, GameState, GameStateMutationResult};
use demon_bluff_logic_engine::{PlayerAction, RevealStrategy, predict};
use itertools::Itertools;

pub fn test_game_state(
	state_name: &str,
	expected_outcome: PlayerAction,
	reveal_strategy: RevealStrategy,
) {
	let mut path = PathBuf::new();
	path.push("tests");
	path.push("game_states");
	path.push(state_name);
	path.set_extension("json");

	let file_reader = File::open(path.as_path())
		.unwrap_or_else(|_| panic!("Unable to open game state file: {}", path.display()));
	let game_state: GameState = serde_json::from_reader(file_reader)
		.unwrap_or_else(|_| panic!("Could not deserialize game state file: {}", path.display()));

	// colog::init();

	let log = log::logger();
	let action = predict(&log, &game_state, reveal_strategy).unwrap_or_else(
		|err: demon_bluff_logic_engine::PredictionError| panic!("Prediction error occured: {err}"),
	);

	assert_eq!(
		expected_outcome,
		*action
			.iter()
			.next()
			.expect("There should have been at least one action!")
	);
	assert_eq!(1, action.len());
}

pub fn run_game(
	game_state: &GameState,
	expected_actions: Vec<Action>,
	log_after: Option<usize>,
	reveal_strategy: RevealStrategy,
) {
	let mut game_state = game_state.clone();
	let total_actions = expected_actions.len();
	for (index, action) in expected_actions.into_iter().enumerate() {
		if let Some(log_after) = log_after
			&& log_after == index
		{
			colog::init();
		}

		let log = log::logger();
		let player_actions =
			predict(&log, &game_state, reveal_strategy).expect("Failed prediction!");

		if index == total_actions - 1 && player_actions.len() > 1 {
			panic!(
				"Last prediction should always be decisive! Got: {}",
				player_actions
					.iter()
					.map(|action| format!("{}", action))
					.join("|")
			);
		}

		let mut found_match = false;
		for player_action in &player_actions {
			if player_action.matches_action(&action) {
				found_match = true;
				break;
			}
		}

		assert!(
			found_match,
			"Unexpected player action predicted on turn #{}! Got: {} - Expected: {}",
			index + 1,
			player_actions
				.iter()
				.map(|action| format!("{}", action))
				.join("|"),
			action
		);

		match game_state
			.mutate(action)
			.expect("Game state mutation failed")
		{
			GameStateMutationResult::Win => {
				assert_eq!(total_actions - 1, index, "Game was won too soon")
			}
			GameStateMutationResult::Loss => panic!("Game was lost!"),
			GameStateMutationResult::Continue => {
				assert_ne!(total_actions - 1, index, "Game should be over by now")
			}
		}
	}
}

pub fn generate_state_file(state_name: &str, game_state: &GameState) {
	let mut path = PathBuf::new();
	path.push("tests");
	path.push("game_states");
	path.push(state_name);
	path.set_extension("json");
	fs::write(path, serde_json::to_string_pretty(&game_state).unwrap())
		.expect("Writing out the state file failed!");
}
