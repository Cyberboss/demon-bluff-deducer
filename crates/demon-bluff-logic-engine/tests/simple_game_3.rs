use demon_bluff_gameplay_engine::{
	game_state::{Action, DrawStats, RevealResult, new_game},
	testimony::{Direction, Testimony},
	villager::{GoodVillager, Minion, VillagerArchetype, VillagerIndex, VillagerInstance},
};
use demon_bluff_logic_engine::{PlayerAction, predict};

#[test]
pub fn simple_game_3() {
	let log = log::logger();

	let mut state = new_game(
		vec![
			VillagerArchetype::GoodVillager(GoodVillager::Lover),
			VillagerArchetype::GoodVillager(GoodVillager::Gemcrafter),
			VillagerArchetype::GoodVillager(GoodVillager::Confessor),
			VillagerArchetype::GoodVillager(GoodVillager::Hunter),
			VillagerArchetype::GoodVillager(GoodVillager::Enlightened),
			VillagerArchetype::Minion(Minion::Minion),
		],
		DrawStats::new(4, 0, 1, 0),
		1,
		false,
	);

	// reveal enlightend
	println!("Prediction 1:");
	let mut prediction = predict(&log, &state).expect("prediction failed??");

	assert_eq!(
		&PlayerAction::TryReveal(VillagerIndex(0)),
		prediction.iter().next().unwrap()
	);

	state
		.mutate(Action::TryReveal(RevealResult::new(
			VillagerIndex(0),
			Some(VillagerInstance::new(
				VillagerArchetype::GoodVillager(GoodVillager::Enlightened),
				Some(Testimony::englightened(
					&VillagerIndex(0),
					Direction::Clockwise,
					state.total_villagers(),
				)),
			)),
		)))
		.expect("Bad mutation?");

	// reveal hunter(fake)
	println!("Prediction 2:");
	prediction = predict(&log, &state).expect("prediction failed??");

	assert_eq!(
		&PlayerAction::TryReveal(VillagerIndex(1)),
		prediction.iter().next().unwrap()
	);

	state
		.mutate(Action::TryReveal(RevealResult::new(
			VillagerIndex(1),
			Some(VillagerInstance::new(
				VillagerArchetype::GoodVillager(GoodVillager::Hunter),
				Some(Testimony::hunter(
					&VillagerIndex(1),
					2,
					state.total_villagers(),
				)),
			)),
		)))
		.expect("Bad mutation?");

	// kill fake hunter
	println!("Prediction 3:");
	prediction = predict(&log, &state).expect("prediction failed??");

	// From Naksu: these are essentially their respective claims (visualization of evil claims showing enlightened isn't claimed evil), they are incompatible meaning one of them is lying. there is only one liar. #2's claim does not allow for #1 to be a liar, therefore #2 is the only option. #3 doesn't matter
	assert_eq!(
		&PlayerAction::TryExecute(VillagerIndex(1)),
		prediction.iter().next().unwrap()
	);
}
