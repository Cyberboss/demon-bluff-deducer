use demon_bluff_gameplay_engine::{
	Expression,
	game_state::{
		Action, DrawStats, GameStateMutationResult, KillAttempt, KillData, KillResult,
		RevealResult, new_game,
	},
	testimony::{ConfessorClaim, Testimony},
	villager::{GoodVillager, Minion, VillagerArchetype, VillagerIndex, VillagerInstance},
};
use demon_bluff_logic_engine::{PlayerAction, predict};

#[test]
pub fn simple_game_2() {
	let log = log::logger();

	let mut state = new_game(
		vec![
			VillagerArchetype::GoodVillager(GoodVillager::Lover),
			VillagerArchetype::GoodVillager(GoodVillager::Gemcrafter),
			VillagerArchetype::GoodVillager(GoodVillager::Confessor),
			VillagerArchetype::GoodVillager(GoodVillager::Hunter),
			VillagerArchetype::Minion(Minion::Minion),
		],
		DrawStats::new(4, 0, 1, 0),
		1,
		false,
	);

	// reveal confessor (fake)
	println!("Prediction 1:");
	let mut prediction = predict(&log, &state).expect("prediction failed??");

	assert_eq!(
		&PlayerAction::TryReveal(VillagerIndex(0)),
		prediction.iter().next().unwrap()
	);

	let mut mutation_result = state
		.mutate(Action::TryReveal(RevealResult::new(
			VillagerIndex(0),
			Some(VillagerInstance::new(
				VillagerArchetype::GoodVillager(GoodVillager::Confessor),
				Some(Expression::Unary(Testimony::Confess(ConfessorClaim::Dizzy))),
			)),
		)))
		.expect("malformed game step??");
	assert_eq!(GameStateMutationResult::Continue, mutation_result);

	// kill confessor
	println!("Prediction 2:");
	prediction = predict(&log, &state).expect("prediction failed??");

	assert_eq!(
		&PlayerAction::TryExecute(VillagerIndex(0)),
		prediction.iter().next().unwrap()
	);

	mutation_result = state
		.mutate(Action::TryExecute(KillAttempt::new(
			VillagerIndex(0),
			Some(KillResult::Revealed(
				KillData::new(Some(VillagerArchetype::Minion(Minion::Minion)), false)
					.expect("This is valid kill data"),
			)),
		)))
		.expect("malformed game step??");
	assert_eq!(GameStateMutationResult::Win, mutation_result);
}
