use demon_bluff_gameplay_engine::{
	Expression,
	game_state::{
		AbilityResult, Action, DrawStats, GameStateMutationResult, KillAttempt, KillData,
		KillResult, RevealResult, new_game,
	},
	testimony::{ConfessorClaim, Direction, RoleClaim, Testimony},
	villager::{GoodVillager, Minion, VillagerArchetype, VillagerIndex, VillagerInstance},
};
use demon_bluff_logic_engine::{PlayerAction, RevealStrategy, predict};

use super::run_game;
use crate::helpers::TestAction;

#[test]
pub fn game_1() {
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

	// reveal lover
	println!("Prediction 1:");
	let mut prediction =
		predict(&log, &state, RevealStrategy::Simple).expect("prediction failed??");

	assert_eq!(
		&PlayerAction::TryReveal(VillagerIndex(0)),
		prediction.iter().next().unwrap()
	);

	let mut mutation_result = state
		.mutate(Action::TryReveal(RevealResult::new(
			VillagerIndex(0),
			Some(VillagerInstance::new(
				VillagerArchetype::GoodVillager(GoodVillager::Lover),
				Some(Testimony::lover(
					&VillagerIndex(0),
					1,
					state.total_villagers(),
				)),
			)),
		)))
		.expect("malformed game step??");
	assert_eq!(GameStateMutationResult::Continue, mutation_result);

	// reveal confessor (fake)
	println!("Prediction 2:");
	prediction = predict(&log, &state, RevealStrategy::Simple).expect("prediction failed??");

	assert_eq!(
		&PlayerAction::TryReveal(VillagerIndex(1)),
		prediction.iter().next().unwrap()
	);

	mutation_result = state
		.mutate(Action::TryReveal(RevealResult::new(
			VillagerIndex(1),
			Some(VillagerInstance::new(
				VillagerArchetype::GoodVillager(GoodVillager::Confessor),
				Some(Expression::Leaf(Testimony::Confess(ConfessorClaim::Dizzy))),
			)),
		)))
		.expect("malformed game step??");
	assert_eq!(GameStateMutationResult::Continue, mutation_result);

	// kill confessor
	println!("Prediction 3:");
	prediction = predict(&log, &state, RevealStrategy::Simple).expect("prediction failed??");

	assert_eq!(
		&PlayerAction::TryExecute(VillagerIndex(1)),
		prediction.iter().next().unwrap()
	);

	mutation_result = state
		.mutate(Action::TryExecute(KillAttempt::new(
			VillagerIndex(1),
			Some(KillResult::Revealed(
				KillData::new(Some(VillagerArchetype::Minion(Minion::Minion)), false)
					.expect("This is valid kill data"),
			)),
		)))
		.expect("malformed game step??");
	assert_eq!(GameStateMutationResult::Win, mutation_result);
}

#[test]
pub fn game_2() {
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
	let mut prediction =
		predict(&log, &state, RevealStrategy::Simple).expect("prediction failed??");

	assert_eq!(
		&PlayerAction::TryReveal(VillagerIndex(0)),
		prediction.iter().next().unwrap()
	);

	let mut mutation_result = state
		.mutate(Action::TryReveal(RevealResult::new(
			VillagerIndex(0),
			Some(VillagerInstance::new(
				VillagerArchetype::GoodVillager(GoodVillager::Confessor),
				Some(Expression::Leaf(Testimony::Confess(ConfessorClaim::Dizzy))),
			)),
		)))
		.expect("malformed game step??");
	assert_eq!(GameStateMutationResult::Continue, mutation_result);

	// kill confessor
	println!("Prediction 2:");
	prediction = predict(&log, &state, RevealStrategy::Simple).expect("prediction failed??");

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

#[test]
pub fn game_3() {
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
	let mut prediction =
		predict(&log, &state, RevealStrategy::Simple).expect("prediction failed??");

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
	prediction = predict(&log, &state, RevealStrategy::Simple).expect("prediction failed??");

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
	prediction = predict(&log, &state, RevealStrategy::Simple).expect("prediction failed??");

	// From Naksu: these are essentially their respective claims (visualization of evil claims showing enlightened isn't claimed evil), they are incompatible meaning one of them is lying. there is only one liar. #2's claim does not allow for #1 to be a liar, therefore #2 is the only option. #3 doesn't matter
	assert_eq!(
		&PlayerAction::TryExecute(VillagerIndex(1)),
		prediction.iter().next().unwrap()
	);
}

#[test]
fn game_4() {
	let game_state = new_game(
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

	run_game(
		&game_state,
		vec![
			TestAction::TryReveal(RevealResult::new(
				VillagerIndex(0),
				Some(VillagerInstance::new(
					VillagerArchetype::GoodVillager(GoodVillager::Hunter),
					Some(Testimony::hunter(
						&VillagerIndex(0),
						2,
						game_state.total_villagers(),
					)),
				)),
			)),
			TestAction::TryReveal(RevealResult::new(
				VillagerIndex(1),
				Some(VillagerInstance::new(
					VillagerArchetype::GoodVillager(GoodVillager::Lover),
					Some(Testimony::lover(
						&VillagerIndex(1),
						1,
						game_state.total_villagers(),
					)),
				)),
			)),
			TestAction::TryReveal(RevealResult::new(
				VillagerIndex(2),
				Some(VillagerInstance::new(
					VillagerArchetype::GoodVillager(GoodVillager::Gemcrafter),
					Some(Expression::Leaf(Testimony::Good(VillagerIndex(2)))),
				)),
			)),
			TestAction::TryReveal(RevealResult::new(
				VillagerIndex(3),
				Some(VillagerInstance::new(
					VillagerArchetype::GoodVillager(GoodVillager::Confessor),
					Some(Expression::Leaf(Testimony::Confess(ConfessorClaim::Good))),
				)),
			)),
			TestAction::TryReveal(RevealResult::new(
				VillagerIndex(4),
				Some(VillagerInstance::new(
					VillagerArchetype::GoodVillager(GoodVillager::Enlightened),
					Some(Testimony::englightened(
						&VillagerIndex(4),
						Direction::CounterClockwise,
						game_state.total_villagers(),
					)),
				)),
			)),
			TestAction::TryExecute(KillAttempt::new(
				VillagerIndex(2),
				Some(KillResult::Revealed(
					KillData::new(Some(VillagerArchetype::Minion(Minion::Minion)), false)
						.expect("Bad kill data"),
				)),
			)),
		],
		None,
	);
}

#[test]
fn game_5() {
	let game_state = new_game(
		vec![
			VillagerArchetype::GoodVillager(GoodVillager::Lover),
			VillagerArchetype::GoodVillager(GoodVillager::Gemcrafter),
			VillagerArchetype::GoodVillager(GoodVillager::Confessor),
			VillagerArchetype::GoodVillager(GoodVillager::Hunter),
			VillagerArchetype::GoodVillager(GoodVillager::Medium),
			VillagerArchetype::GoodVillager(GoodVillager::Judge),
			VillagerArchetype::Minion(Minion::Minion),
		],
		DrawStats::new(5, 0, 1, 0),
		1,
		false,
	);

	run_game(
		&game_state,
		vec![
			TestAction::TryReveal(RevealResult::new(
				VillagerIndex(0),
				Some(VillagerInstance::new(
					VillagerArchetype::GoodVillager(GoodVillager::Hunter),
					Some(Testimony::hunter(
						&VillagerIndex(0),
						2,
						game_state.total_villagers(),
					)),
				)),
			)),
			TestAction::TryReveal(RevealResult::new(
				VillagerIndex(1),
				Some(VillagerInstance::new(
					VillagerArchetype::GoodVillager(GoodVillager::Confessor),
					Some(Expression::Leaf(Testimony::Confess(ConfessorClaim::Good))),
				)),
			)),
			TestAction::TryReveal(RevealResult::new(
				VillagerIndex(2),
				Some(VillagerInstance::new(
					VillagerArchetype::GoodVillager(GoodVillager::Medium),
					Some(Expression::Leaf(Testimony::Role(RoleClaim::new(
						VillagerIndex(5),
						VillagerArchetype::GoodVillager(GoodVillager::Lover),
					)))),
				)),
			)),
			TestAction::TryReveal(RevealResult::new(
				VillagerIndex(3),
				Some(VillagerInstance::new(
					VillagerArchetype::GoodVillager(GoodVillager::Judge),
					None,
				)),
			)),
			TestAction::TryReveal(RevealResult::new(
				VillagerIndex(4),
				Some(VillagerInstance::new(
					VillagerArchetype::GoodVillager(GoodVillager::Gemcrafter),
					Some(Expression::Leaf(Testimony::Good(VillagerIndex(4)))),
				)),
			)),
			TestAction::TryReveal(RevealResult::new(
				VillagerIndex(5),
				Some(VillagerInstance::new(
					VillagerArchetype::GoodVillager(GoodVillager::Lover),
					Some(Testimony::lover(
						&VillagerIndex(5),
						1,
						game_state.total_villagers(),
					)),
				)),
			)),
			TestAction::Ability(
				vec![VillagerIndex::number(1)],
				AbilityResult::new(
					VillagerIndex(3),
					Some(Expression::Not(Box::new(Expression::Leaf(
						Testimony::Lying(VillagerIndex(0)),
					)))),
					None,
				),
			),
			TestAction::TryExecute(KillAttempt::new(
				VillagerIndex(4),
				Some(KillResult::Revealed(
					KillData::new(Some(VillagerArchetype::Minion(Minion::Minion)), false)
						.expect("Bad kill data"),
				)),
			)),
		],
		None,
	);
}

#[test]
fn game_6() {
	let game_state = new_game(
		vec![
			VillagerArchetype::GoodVillager(GoodVillager::Lover),
			VillagerArchetype::GoodVillager(GoodVillager::Gemcrafter),
			VillagerArchetype::GoodVillager(GoodVillager::Confessor),
			VillagerArchetype::GoodVillager(GoodVillager::Hunter),
			VillagerArchetype::GoodVillager(GoodVillager::Medium),
			VillagerArchetype::GoodVillager(GoodVillager::Judge),
			VillagerArchetype::Minion(Minion::Minion),
		],
		DrawStats::new(5, 0, 1, 0),
		1,
		false,
	);

	run_game(
		&game_state,
		vec![
			TestAction::TryReveal(RevealResult::new(
				VillagerIndex(0),
				Some(VillagerInstance::new(
					VillagerArchetype::GoodVillager(GoodVillager::Hunter),
					Some(Testimony::hunter(
						&VillagerIndex(0),
						2,
						game_state.total_villagers(),
					)),
				)),
			)),
			TestAction::TryReveal(RevealResult::new(
				VillagerIndex(1),
				Some(VillagerInstance::new(
					VillagerArchetype::GoodVillager(GoodVillager::Confessor),
					Some(Expression::Leaf(Testimony::Confess(ConfessorClaim::Good))),
				)),
			)),
			TestAction::TryReveal(RevealResult::new(
				VillagerIndex(2),
				Some(VillagerInstance::new(
					VillagerArchetype::GoodVillager(GoodVillager::Medium),
					Some(Expression::Leaf(Testimony::Role(RoleClaim::new(
						VillagerIndex(5),
						VillagerArchetype::GoodVillager(GoodVillager::Lover),
					)))),
				)),
			)),
			TestAction::TryReveal(RevealResult::new(
				VillagerIndex(3),
				Some(VillagerInstance::new(
					VillagerArchetype::GoodVillager(GoodVillager::Judge),
					None,
				)),
			)),
			TestAction::TryReveal(RevealResult::new(
				VillagerIndex(4),
				Some(VillagerInstance::new(
					VillagerArchetype::GoodVillager(GoodVillager::Gemcrafter),
					Some(Expression::Leaf(Testimony::Good(VillagerIndex(4)))),
				)),
			)),
			TestAction::TryReveal(RevealResult::new(
				VillagerIndex(5),
				Some(VillagerInstance::new(
					VillagerArchetype::GoodVillager(GoodVillager::Lover),
					Some(Testimony::lover(
						&VillagerIndex(5),
						1,
						game_state.total_villagers(),
					)),
				)),
			)),
			TestAction::Ability(
				vec![VillagerIndex::number(0)],
				AbilityResult::new(
					VillagerIndex(3),
					Some(Expression::Leaf(Testimony::Lying(VillagerIndex(0)))),
					None,
				),
			),
			TestAction::TryExecute(KillAttempt::new(
				VillagerIndex(0),
				Some(KillResult::Revealed(
					KillData::new(Some(VillagerArchetype::Minion(Minion::Minion)), false)
						.expect("Bad kill data"),
				)),
			)),
		],
		None,
	);
}

#[test]
fn game_7() {
	let game_state = new_game(
		vec![
			VillagerArchetype::GoodVillager(GoodVillager::Lover),
			VillagerArchetype::GoodVillager(GoodVillager::Gemcrafter),
			VillagerArchetype::GoodVillager(GoodVillager::Confessor),
			VillagerArchetype::GoodVillager(GoodVillager::Hunter),
			VillagerArchetype::GoodVillager(GoodVillager::Medium),
			VillagerArchetype::GoodVillager(GoodVillager::Judge),
			VillagerArchetype::Minion(Minion::Minion),
		],
		DrawStats::new(5, 0, 1, 0),
		1,
		false,
	);

	run_game(
		&game_state,
		vec![
			TestAction::TryReveal(RevealResult::new(
				VillagerIndex(0),
				Some(VillagerInstance::new(
					VillagerArchetype::GoodVillager(GoodVillager::Hunter),
					Some(Testimony::hunter(
						&VillagerIndex(0),
						2,
						game_state.total_villagers(),
					)),
				)),
			)),
			TestAction::TryReveal(RevealResult::new(
				VillagerIndex(1),
				Some(VillagerInstance::new(
					VillagerArchetype::GoodVillager(GoodVillager::Confessor),
					Some(Expression::Leaf(Testimony::Confess(ConfessorClaim::Good))),
				)),
			)),
			TestAction::TryReveal(RevealResult::new(
				VillagerIndex(2),
				Some(VillagerInstance::new(
					VillagerArchetype::GoodVillager(GoodVillager::Medium),
					Some(Expression::Leaf(Testimony::Role(RoleClaim::new(
						VillagerIndex(5),
						VillagerArchetype::GoodVillager(GoodVillager::Lover),
					)))),
				)),
			)),
			TestAction::TryReveal(RevealResult::new(
				VillagerIndex(3),
				Some(VillagerInstance::new(
					VillagerArchetype::GoodVillager(GoodVillager::Judge),
					None,
				)),
			)),
			TestAction::TryReveal(RevealResult::new(
				VillagerIndex(4),
				Some(VillagerInstance::new(
					VillagerArchetype::GoodVillager(GoodVillager::Gemcrafter),
					Some(Expression::Leaf(Testimony::Good(VillagerIndex(4)))),
				)),
			)),
			TestAction::TryReveal(RevealResult::new(
				VillagerIndex(5),
				Some(VillagerInstance::new(
					VillagerArchetype::GoodVillager(GoodVillager::Lover),
					Some(Testimony::lover(
						&VillagerIndex(5),
						1,
						game_state.total_villagers(),
					)),
				)),
			)),
			TestAction::Ability(
				vec![VillagerIndex::number(4)],
				AbilityResult::new(
					VillagerIndex(3),
					Some(Expression::Leaf(Testimony::Lying(VillagerIndex(4)))),
					None,
				),
			),
			TestAction::TryExecute(KillAttempt::new(
				VillagerIndex(4),
				Some(KillResult::Revealed(
					KillData::new(Some(VillagerArchetype::Minion(Minion::Minion)), false)
						.expect("Bad kill data"),
				)),
			)),
		],
		None,
	);
}

#[test]
fn game_8() {
	let game_state = new_game(
		vec![
			VillagerArchetype::GoodVillager(GoodVillager::Lover),
			VillagerArchetype::GoodVillager(GoodVillager::Gemcrafter),
			VillagerArchetype::GoodVillager(GoodVillager::Confessor),
			VillagerArchetype::GoodVillager(GoodVillager::Hunter),
			VillagerArchetype::GoodVillager(GoodVillager::Medium),
			VillagerArchetype::GoodVillager(GoodVillager::Judge),
			VillagerArchetype::Minion(Minion::Minion),
		],
		DrawStats::new(5, 0, 1, 0),
		1,
		false,
	);

	run_game(
		&game_state,
		vec![
			TestAction::TryReveal(RevealResult::new(
				VillagerIndex(0),
				Some(VillagerInstance::new(
					VillagerArchetype::GoodVillager(GoodVillager::Hunter),
					Some(Testimony::hunter(
						&VillagerIndex(0),
						2,
						game_state.total_villagers(),
					)),
				)),
			)),
			TestAction::TryReveal(RevealResult::new(
				VillagerIndex(1),
				Some(VillagerInstance::new(
					VillagerArchetype::GoodVillager(GoodVillager::Confessor),
					Some(Expression::Leaf(Testimony::Confess(ConfessorClaim::Good))),
				)),
			)),
			TestAction::TryReveal(RevealResult::new(
				VillagerIndex(2),
				Some(VillagerInstance::new(
					VillagerArchetype::GoodVillager(GoodVillager::Medium),
					Some(Expression::Leaf(Testimony::Role(RoleClaim::new(
						VillagerIndex(5),
						VillagerArchetype::GoodVillager(GoodVillager::Lover),
					)))),
				)),
			)),
			TestAction::TryReveal(RevealResult::new(
				VillagerIndex(3),
				Some(VillagerInstance::new(
					VillagerArchetype::GoodVillager(GoodVillager::Judge),
					None,
				)),
			)),
			TestAction::TryReveal(RevealResult::new(
				VillagerIndex(4),
				Some(VillagerInstance::new(
					VillagerArchetype::GoodVillager(GoodVillager::Gemcrafter),
					Some(Expression::Leaf(Testimony::Good(VillagerIndex(4)))),
				)),
			)),
			TestAction::TryReveal(RevealResult::new(
				VillagerIndex(5),
				Some(VillagerInstance::new(
					VillagerArchetype::GoodVillager(GoodVillager::Lover),
					Some(Testimony::lover(
						&VillagerIndex(5),
						1,
						game_state.total_villagers(),
					)),
				)),
			)),
			TestAction::Ability(
				vec![VillagerIndex::number(4)],
				AbilityResult::new(
					VillagerIndex(3),
					Some(Expression::Not(Box::new(Expression::Leaf(
						Testimony::Lying(VillagerIndex(4)),
					)))),
					None,
				),
			),
			TestAction::TryExecute(KillAttempt::new(
				VillagerIndex(0),
				Some(KillResult::Revealed(
					KillData::new(Some(VillagerArchetype::Minion(Minion::Minion)), false)
						.expect("Bad kill data"),
				)),
			)),
		],
		None,
	);
}

#[test]
fn game_9() {
	let game_state = new_game(
		vec![
			VillagerArchetype::GoodVillager(GoodVillager::Lover),
			VillagerArchetype::GoodVillager(GoodVillager::Gemcrafter),
			VillagerArchetype::GoodVillager(GoodVillager::Confessor),
			VillagerArchetype::GoodVillager(GoodVillager::Hunter),
			VillagerArchetype::GoodVillager(GoodVillager::Medium),
			VillagerArchetype::GoodVillager(GoodVillager::Judge),
			VillagerArchetype::Minion(Minion::Minion),
		],
		DrawStats::new(5, 0, 1, 0),
		1,
		false,
	);

	run_game(
		&game_state,
		vec![
			TestAction::TryReveal(RevealResult::new(
				VillagerIndex(0),
				Some(VillagerInstance::new(
					VillagerArchetype::GoodVillager(GoodVillager::Hunter),
					Some(Testimony::hunter(
						&VillagerIndex(0),
						2,
						game_state.total_villagers(),
					)),
				)),
			)),
			TestAction::TryReveal(RevealResult::new(
				VillagerIndex(1),
				Some(VillagerInstance::new(
					VillagerArchetype::GoodVillager(GoodVillager::Confessor),
					Some(Expression::Leaf(Testimony::Confess(ConfessorClaim::Good))),
				)),
			)),
			TestAction::TryReveal(RevealResult::new(
				VillagerIndex(2),
				Some(VillagerInstance::new(
					VillagerArchetype::GoodVillager(GoodVillager::Medium),
					Some(Expression::Leaf(Testimony::Role(RoleClaim::new(
						VillagerIndex(5),
						VillagerArchetype::GoodVillager(GoodVillager::Lover),
					)))),
				)),
			)),
			TestAction::TryReveal(RevealResult::new(
				VillagerIndex(3),
				Some(VillagerInstance::new(
					VillagerArchetype::GoodVillager(GoodVillager::Judge),
					None,
				)),
			)),
			TestAction::TryReveal(RevealResult::new(
				VillagerIndex(4),
				Some(VillagerInstance::new(
					VillagerArchetype::GoodVillager(GoodVillager::Gemcrafter),
					Some(Expression::Leaf(Testimony::Good(VillagerIndex(4)))),
				)),
			)),
			TestAction::TryReveal(RevealResult::new(
				VillagerIndex(5),
				Some(VillagerInstance::new(
					VillagerArchetype::GoodVillager(GoodVillager::Lover),
					Some(Testimony::lover(
						&VillagerIndex(5),
						1,
						game_state.total_villagers(),
					)),
				)),
			)),
			TestAction::Ability(
				vec![VillagerIndex::number(4)],
				AbilityResult::new(
					VillagerIndex(3),
					Some(Expression::Not(Box::new(Expression::Leaf(
						Testimony::Lying(VillagerIndex(4)),
					)))),
					None,
				),
			),
			TestAction::TryExecute(KillAttempt::new(
				VillagerIndex(0),
				Some(KillResult::Revealed(
					KillData::new(Some(VillagerArchetype::Minion(Minion::Minion)), false)
						.expect("Bad kill data"),
				)),
			)),
		],
		None,
	);
}

#[test]
fn game_10() {
	let game_state = new_game(
		vec![
			VillagerArchetype::GoodVillager(GoodVillager::Lover),
			VillagerArchetype::GoodVillager(GoodVillager::Enlightened),
			VillagerArchetype::GoodVillager(GoodVillager::Hunter),
			VillagerArchetype::GoodVillager(GoodVillager::Medium),
			VillagerArchetype::GoodVillager(GoodVillager::Judge),
			VillagerArchetype::Minion(Minion::Minion),
		],
		DrawStats::new(5, 0, 1, 0),
		1,
		false,
	);

	run_game(
		&game_state,
		vec![
			TestAction::TryReveal(RevealResult::new(
				VillagerIndex(0),
				Some(VillagerInstance::new(
					VillagerArchetype::GoodVillager(GoodVillager::Lover),
					Some(Testimony::lover(
						&VillagerIndex(0),
						1,
						game_state.total_villagers(),
					)),
				)),
			)),
			TestAction::TryReveal(RevealResult::new(
				VillagerIndex(1),
				Some(VillagerInstance::new(
					VillagerArchetype::GoodVillager(GoodVillager::Medium),
					Some(Expression::Leaf(Testimony::Role(RoleClaim::new(
						VillagerIndex(2),
						VillagerArchetype::GoodVillager(GoodVillager::Hunter),
					)))),
				)),
			)),
			TestAction::TryReveal(RevealResult::new(
				VillagerIndex(2),
				Some(VillagerInstance::new(
					VillagerArchetype::GoodVillager(GoodVillager::Hunter),
					Some(Testimony::hunter(
						&VillagerIndex(2),
						2,
						game_state.total_villagers(),
					)),
				)),
			)),
			TestAction::TryExecute(KillAttempt::new(
				VillagerIndex(0),
				Some(KillResult::Revealed(
					KillData::new(Some(VillagerArchetype::Minion(Minion::Minion)), false)
						.expect("Bad kill data"),
				)),
			)),
		],
		None,
	);
}
