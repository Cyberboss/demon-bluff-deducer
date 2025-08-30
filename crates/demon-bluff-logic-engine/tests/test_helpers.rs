use std::{fs::File, path::PathBuf};

use demon_bluff_gameplay_engine::game_state::GameState;
use demon_bluff_logic_engine::{PlayerAction, predict};

pub fn test_game_state(state_name: &str, expected_outcome: PlayerAction) {
	let mut path = PathBuf::new();
	path.push("tests");
	path.push("game_states");
	path.push(state_name);
	path.set_extension("json");

	let file_reader = File::open(path.as_path())
		.expect(format!("Unable to open game state file: {}", path.display()).as_str());
	let game_state: GameState = serde_json::from_reader(file_reader)
		.expect(format!("Could not deserialize game state file: {}", path.display()).as_str());

	colog::init();

	let log = log::logger();
	let action = predict(&log, &game_state)
		.unwrap_or_else(|err| panic!("Prediction error occured: {}", err));

	assert_eq!(
		expected_outcome,
		*action
			.iter()
			.next()
			.expect("There should have been at least one action!")
	);
	assert_eq!(1, action.len());
}
