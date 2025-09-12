use std::{
	fmt::{Display, write},
	fs::{self, File},
	path::PathBuf,
	sync::OnceLock,
	time::Instant,
};

use demon_bluff_gameplay_engine::{
	game_state::{
		AbilityResult, Action, GameState, GameStateMutationResult, KillAttempt, RevealResult,
	},
	villager::VillagerIndex,
};
use demon_bluff_logic_engine::{AbilityAttempt, PlayerAction, RevealStrategy, predict};
use itertools::Itertools;
use log::info;

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

#[derive(Debug, Clone)]
pub enum TestAction {
	TryExecute(KillAttempt),
	TryReveal(RevealResult),
	Ability(Vec<VillagerIndex>, AbilityResult),
}

pub fn run_game(
	game_state: &GameState,
	expected_actions: Vec<TestAction>,
	log_after: Option<usize>,
	reveal_strategy: RevealStrategy,
) {
	let mut game_state = game_state.clone();
	let total_actions = expected_actions.len();
	let mut log = log::logger();

	for (index, action) in expected_actions.into_iter().enumerate() {
		if let Some(log_after) = log_after
			&& log_after == index
		{
			colog::default_builder()
				//.filter_level(log::LevelFilter::Debug)
				.init();
			log = log::logger();
		}

		info!(logger: log::logger(), "Starting turn #{}", index + 1);
		let start_time = Instant::now();
		let player_actions =
			predict(&log, &game_state, reveal_strategy).expect("Failed prediction!");
		let end_time = Instant::now();

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
			if action.matches_action(player_action) {
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

		info!(
			"Prediction took {:.2}s: {}{}",
			end_time.duration_since(start_time).as_secs_f32(),
			player_actions
				.iter()
				.map(|action| format!("{}", action))
				.join("|"),
			if player_actions.len() > 1 {
				format!(" - Selecting: {}", action)
			} else {
				"".to_string()
			}
		);

		match game_state
			.mutate(action.into())
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

impl TestAction {
	pub fn matches_action(&self, player_action: &PlayerAction) -> bool {
		match self {
			Self::TryExecute(kill_attempt) => {
				if let PlayerAction::TryExecute(action_attempt) = player_action {
					kill_attempt.target() == action_attempt
				} else {
					false
				}
			}
			Self::TryReveal(reveal_result) => {
				if let PlayerAction::TryReveal(action_attempt) = player_action {
					reveal_result.index() == action_attempt
				} else {
					false
				}
			}
			Self::Ability(items, ability_result) => {
				if let PlayerAction::Ability(ability_attempt) = player_action {
					ability_attempt.source() == ability_result.source()
						&& items.iter().all(|villager_index| {
							ability_attempt.targets().contains(villager_index)
						})
				} else {
					false
				}
			}
		}
	}
}

impl Display for TestAction {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", PlayerAction::from(self.clone()))
	}
}

impl From<TestAction> for Action {
	fn from(value: TestAction) -> Self {
		match value {
			TestAction::TryExecute(kill_attempt) => Action::TryExecute(kill_attempt),
			TestAction::TryReveal(reveal_result) => Action::TryReveal(reveal_result),
			TestAction::Ability(_, ability_result) => Action::Ability(ability_result),
		}
	}
}

impl From<TestAction> for PlayerAction {
	fn from(value: TestAction) -> Self {
		match value {
			TestAction::TryExecute(kill_attempt) => {
				PlayerAction::TryExecute(kill_attempt.target().clone())
			}
			TestAction::TryReveal(reveal_result) => {
				PlayerAction::TryReveal(reveal_result.index().clone())
			}
			TestAction::Ability(items, ability_result) => PlayerAction::Ability(
				AbilityAttempt::new(ability_result.source().clone(), items.into_iter().collect()),
			),
		}
	}
}
