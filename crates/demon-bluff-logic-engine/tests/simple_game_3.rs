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

	colog::init();
	let log = log::logger();

	// reveal hunter
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
}
