use demon_bluff_gameplay_engine::{
	Expression,
	game_state::{
		AbilityResult, Action, DrawStats, KillAttempt, KillData, KillResult, RevealResult,
		UnrevealedKillData, new_game,
	},
	testimony::{ConfessorClaim, Direction, RoleClaim, ScoutClaim, Testimony},
	villager::{
		Demon, GoodVillager, Minion, Outcast, VillagerArchetype, VillagerIndex, VillagerInstance,
	},
};

use super::run_game;
use crate::helpers::TestAction;

#[test]
fn game_11() {
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

#[test]
fn game_12() {
	let game_state = new_game(
		vec![
			VillagerArchetype::GoodVillager(GoodVillager::Enlightened),
			VillagerArchetype::GoodVillager(GoodVillager::Medium),
			VillagerArchetype::GoodVillager(GoodVillager::Jester),
			VillagerArchetype::GoodVillager(GoodVillager::Empress),
			VillagerArchetype::GoodVillager(GoodVillager::Confessor),
			VillagerArchetype::Outcast(Outcast::Wretch),
			VillagerArchetype::Minion(Minion::Minion),
			VillagerArchetype::Minion(Minion::Twinion),
		],
		DrawStats::new(4, 1, 2, 0),
		2,
		false,
	);

	run_game(
		&game_state,
		vec![
			TestAction::TryReveal(RevealResult::new(
				VillagerIndex::number(1),
				Some(VillagerInstance::new(
					VillagerArchetype::GoodVillager(GoodVillager::Empress),
					Some(Testimony::empress(&[
						VillagerIndex::number(7),
						VillagerIndex::number(4),
						VillagerIndex::number(3),
					])),
				)),
			)),
			TestAction::TryReveal(RevealResult::new(
				VillagerIndex::number(2),
				Some(VillagerInstance::new(
					VillagerArchetype::GoodVillager(GoodVillager::Enlightened),
					Some(Testimony::englightened(
						&VillagerIndex::number(2),
						Direction::Clockwise,
						game_state.total_villagers(),
					)),
				)),
			)),
			TestAction::TryReveal(RevealResult::new(
				VillagerIndex::number(3),
				Some(VillagerInstance::new(
					VillagerArchetype::Outcast(Outcast::Wretch),
					Some(Expression::Leaf(Testimony::FakeEvil(
						VillagerIndex::number(3),
					))),
				)),
			)),
			TestAction::TryReveal(RevealResult::new(
				VillagerIndex::number(4),
				Some(VillagerInstance::new(
					VillagerArchetype::GoodVillager(GoodVillager::Medium),
					Some(Expression::Leaf(Testimony::Role(RoleClaim::new(
						VillagerIndex::number(7),
						VillagerArchetype::GoodVillager(GoodVillager::Confessor),
					)))),
				)),
			)),
			TestAction::TryReveal(RevealResult::new(
				VillagerIndex::number(5),
				Some(VillagerInstance::new(
					VillagerArchetype::GoodVillager(GoodVillager::Medium),
					Some(Expression::Leaf(Testimony::Role(RoleClaim::new(
						VillagerIndex::number(6),
						VillagerArchetype::GoodVillager(GoodVillager::Jester),
					)))),
				)),
			)),
			TestAction::TryReveal(RevealResult::new(
				VillagerIndex::number(6),
				Some(VillagerInstance::new(
					VillagerArchetype::GoodVillager(GoodVillager::Jester),
					None,
				)),
			)),
			TestAction::TryExecute(KillAttempt::new(
				VillagerIndex::number(6),
				Some(KillResult::Revealed(
					KillData::new(Some(VillagerArchetype::Minion(Minion::Twinion)), false)
						.expect("Bad kill data?"),
				)),
			)),
			TestAction::TryExecute(KillAttempt::new(
				VillagerIndex::number(5),
				Some(KillResult::Revealed(
					KillData::new(Some(VillagerArchetype::Minion(Minion::Minion)), false)
						.expect("Bad kill data?"),
				)),
			)),
		],
		None,
	);
}

#[test]
fn game_13() {
	let game_state = new_game(
		vec![
			VillagerArchetype::GoodVillager(GoodVillager::Jester),
			VillagerArchetype::GoodVillager(GoodVillager::Medium),
			VillagerArchetype::GoodVillager(GoodVillager::Gemcrafter),
			VillagerArchetype::GoodVillager(GoodVillager::Knight),
			VillagerArchetype::GoodVillager(GoodVillager::Empress),
			VillagerArchetype::GoodVillager(GoodVillager::Hunter),
			VillagerArchetype::Outcast(Outcast::Wretch),
			VillagerArchetype::Minion(Minion::Minion),
			VillagerArchetype::Minion(Minion::Twinion),
		],
		DrawStats::new(5, 1, 2, 0),
		2,
		false,
	);

	run_game(
		&game_state,
		vec![
			TestAction::TryReveal(RevealResult::new(
				VillagerIndex::number(1),
				Some(VillagerInstance::new(
					VillagerArchetype::GoodVillager(GoodVillager::Knight),
					Some(Expression::Leaf(Testimony::Invincible(
						VillagerIndex::number(1),
					))),
				)),
			)),
			TestAction::TryReveal(RevealResult::new(
				VillagerIndex::number(2),
				Some(VillagerInstance::new(
					VillagerArchetype::GoodVillager(GoodVillager::Medium),
					Some(Expression::Leaf(Testimony::Role(RoleClaim::new(
						VillagerIndex::number(3),
						VillagerArchetype::GoodVillager(GoodVillager::Hunter),
					)))),
				)),
			)),
			TestAction::TryReveal(RevealResult::new(
				VillagerIndex::number(3),
				Some(VillagerInstance::new(
					VillagerArchetype::GoodVillager(GoodVillager::Hunter),
					Some(Testimony::hunter(
						&VillagerIndex::number(3),
						3,
						game_state.total_villagers(),
					)),
				)),
			)),
			TestAction::TryReveal(RevealResult::new(
				VillagerIndex::number(4),
				Some(VillagerInstance::new(
					VillagerArchetype::GoodVillager(GoodVillager::Jester),
					None,
				)),
			)),
			TestAction::TryReveal(RevealResult::new(
				VillagerIndex::number(5),
				Some(VillagerInstance::new(
					VillagerArchetype::GoodVillager(GoodVillager::Gemcrafter),
					Some(Expression::Leaf(Testimony::Good(VillagerIndex::number(8)))),
				)),
			)),
			TestAction::TryReveal(RevealResult::new(
				VillagerIndex::number(6),
				Some(VillagerInstance::new(
					VillagerArchetype::GoodVillager(GoodVillager::Medium),
					Some(Expression::Leaf(Testimony::Role(RoleClaim::new(
						VillagerIndex::number(5),
						VillagerArchetype::GoodVillager(GoodVillager::Gemcrafter),
					)))),
				)),
			)),
			TestAction::TryExecute(KillAttempt::new(
				VillagerIndex::number(2),
				Some(KillResult::Revealed(
					KillData::new(Some(VillagerArchetype::Minion(Minion::Minion)), false)
						.expect("Bad kill data?"),
				)),
			)),
			TestAction::TryExecute(KillAttempt::new(
				VillagerIndex::number(3),
				Some(KillResult::Revealed(
					KillData::new(Some(VillagerArchetype::Minion(Minion::Twinion)), false)
						.expect("Bad kill data?"),
				)),
			)),
		],
		None,
	);
}

#[test]
fn game_14() {
	let game_state = new_game(
		vec![
			VillagerArchetype::GoodVillager(GoodVillager::Empress),
			VillagerArchetype::GoodVillager(GoodVillager::Judge),
			VillagerArchetype::GoodVillager(GoodVillager::Knight),
			VillagerArchetype::GoodVillager(GoodVillager::Jester),
			VillagerArchetype::GoodVillager(GoodVillager::Scout),
			VillagerArchetype::GoodVillager(GoodVillager::Enlightened),
			VillagerArchetype::Outcast(Outcast::Bombardier),
			VillagerArchetype::Minion(Minion::Minion),
			VillagerArchetype::Minion(Minion::Witch),
		],
		DrawStats::new(5, 1, 2, 0),
		2,
		false,
	);

	run_game(
		&game_state,
		vec![
			TestAction::TryReveal(RevealResult::new(
				VillagerIndex::number(1),
				Some(VillagerInstance::new(
					VillagerArchetype::GoodVillager(GoodVillager::Empress),
					Some(Testimony::empress(&[
						VillagerIndex::number(4),
						VillagerIndex::number(7),
						VillagerIndex::number(6),
					])),
				)),
			)),
			TestAction::TryReveal(RevealResult::new(
				VillagerIndex::number(2),
				Some(VillagerInstance::new(
					VillagerArchetype::GoodVillager(GoodVillager::Knight),
					Some(Expression::Leaf(Testimony::Invincible(
						VillagerIndex::number(2),
					))),
				)),
			)),
			TestAction::TryReveal(RevealResult::new(
				VillagerIndex::number(3),
				Some(VillagerInstance::new(
					VillagerArchetype::GoodVillager(GoodVillager::Jester),
					None,
				)),
			)),
			TestAction::TryReveal(RevealResult::new(
				VillagerIndex::number(4),
				Some(VillagerInstance::new(
					VillagerArchetype::GoodVillager(GoodVillager::Enlightened),
					Some(Testimony::englightened(
						&VillagerIndex::number(4),
						Direction::CounterClockwise,
						game_state.total_villagers(),
					)),
				)),
			)),
			TestAction::TryReveal(RevealResult::new(
				VillagerIndex::number(5),
				Some(VillagerInstance::new(
					VillagerArchetype::GoodVillager(GoodVillager::Scout),
					Some(Expression::Leaf(Testimony::Scout(ScoutClaim::new(
						VillagerArchetype::Minion(Minion::Witch),
						3,
					)))),
				)),
			)),
			TestAction::TryReveal(RevealResult::new(
				VillagerIndex::number(6),
				Some(VillagerInstance::new(
					VillagerArchetype::GoodVillager(GoodVillager::Judge),
					None,
				)),
			)),
			TestAction::TryReveal(RevealResult::new(
				VillagerIndex::number(7),
				Some(VillagerInstance::new(
					VillagerArchetype::GoodVillager(GoodVillager::Judge),
					None,
				)),
			)),
			TestAction::TryReveal(RevealResult::new(VillagerIndex::number(8), None)),
			TestAction::Ability(
				vec![
					VillagerIndex::number(1),
					VillagerIndex::number(2),
					VillagerIndex::number(3),
				],
				AbilityResult::new(
					VillagerIndex::number(3),
					Some(Testimony::jester(
						&[
							VillagerIndex::number(1),
							VillagerIndex::number(2),
							VillagerIndex::number(3),
						],
						2,
					)),
					None,
				),
			),
			TestAction::TryExecute(KillAttempt::new(
				VillagerIndex::number(6),
				Some(KillResult::Revealed(
					KillData::new(Some(VillagerArchetype::Minion(Minion::Witch)), false)
						.expect("Bad kill data?"),
				)),
			)),
			TestAction::TryExecute(KillAttempt::new(
				VillagerIndex::number(3),
				Some(KillResult::Revealed(
					KillData::new(Some(VillagerArchetype::Minion(Minion::Minion)), false)
						.expect("Bad kill data?"),
				)),
			)),
		],
		None,
	);
}

#[test]
fn game_15() {
	let game_state = new_game(
		vec![
			VillagerArchetype::GoodVillager(GoodVillager::Medium),
			VillagerArchetype::GoodVillager(GoodVillager::Scout),
			VillagerArchetype::GoodVillager(GoodVillager::Gemcrafter),
			VillagerArchetype::GoodVillager(GoodVillager::Empress),
			VillagerArchetype::GoodVillager(GoodVillager::Enlightened),
			VillagerArchetype::GoodVillager(GoodVillager::Hunter),
			VillagerArchetype::Outcast(Outcast::Wretch),
			VillagerArchetype::Outcast(Outcast::Bombardier),
			VillagerArchetype::Minion(Minion::Minion),
			VillagerArchetype::Demon(Demon::Baa),
		],
		DrawStats::new(5, 1, 1, 1),
		2,
		false,
	);

	run_game(
		&game_state,
		vec![
			TestAction::TryReveal(RevealResult::new(
				VillagerIndex::number(1),
				Some(VillagerInstance::new(
					VillagerArchetype::GoodVillager(GoodVillager::Empress),
					Some(Testimony::empress(&[
						VillagerIndex::number(4),
						VillagerIndex::number(8),
						VillagerIndex::number(2),
					])),
				)),
			)),
			TestAction::TryReveal(RevealResult::new(
				VillagerIndex::number(2),
				Some(VillagerInstance::new(
					VillagerArchetype::GoodVillager(GoodVillager::Hunter),
					Some(Testimony::hunter(
						&VillagerIndex::number(2),
						1,
						game_state.total_villagers(),
					)),
				)),
			)),
			TestAction::TryReveal(RevealResult::new(
				VillagerIndex::number(3),
				Some(VillagerInstance::new(
					VillagerArchetype::GoodVillager(GoodVillager::Enlightened),
					Some(Testimony::englightened(
						&VillagerIndex::number(3),
						Direction::Equidistant,
						game_state.total_villagers(),
					)),
				)),
			)),
			TestAction::TryReveal(RevealResult::new(
				VillagerIndex::number(4),
				Some(VillagerInstance::new(
					VillagerArchetype::GoodVillager(GoodVillager::Gemcrafter),
					Some(Expression::Leaf(Testimony::Good(VillagerIndex::number(8)))),
				)),
			)),
			TestAction::TryReveal(RevealResult::new(
				VillagerIndex::number(5),
				Some(VillagerInstance::new(
					VillagerArchetype::Outcast(Outcast::Wretch),
					Some(Expression::Leaf(Testimony::FakeEvil(
						VillagerIndex::number(5),
					))),
				)),
			)),
			TestAction::TryReveal(RevealResult::new(
				VillagerIndex::number(6),
				Some(VillagerInstance::new(
					VillagerArchetype::GoodVillager(GoodVillager::Medium),
					Some(Expression::Leaf(Testimony::Role(RoleClaim::new(
						VillagerIndex::number(8),
						VillagerArchetype::GoodVillager(GoodVillager::Scout),
					)))),
				)),
			)),
			TestAction::TryReveal(RevealResult::new(
				VillagerIndex::number(7),
				Some(VillagerInstance::new(
					VillagerArchetype::GoodVillager(GoodVillager::Enlightened),
					Some(Testimony::englightened(
						&VillagerIndex::number(7),
						Direction::Clockwise,
						game_state.total_villagers(),
					)),
				)),
			)),
			TestAction::TryExecute(KillAttempt::new(
				VillagerIndex::number(1),
				Some(KillResult::Revealed(
					KillData::new(Some(VillagerArchetype::Minion(Minion::Minion)), false)
						.expect("Bad kill data?"),
				)),
			)),
			TestAction::TryExecute(KillAttempt::new(
				VillagerIndex::number(7),
				Some(KillResult::Revealed(
					KillData::new(Some(VillagerArchetype::Demon(Demon::Baa)), false)
						.expect("Bad kill data?"),
				)),
			)),
		],
		None,
	);
}

#[test]
fn game_16() {
	let game_state = new_game(
		vec![
			VillagerArchetype::GoodVillager(GoodVillager::Scout),
			VillagerArchetype::GoodVillager(GoodVillager::Lover),
			VillagerArchetype::GoodVillager(GoodVillager::Medium),
			VillagerArchetype::GoodVillager(GoodVillager::Confessor),
			VillagerArchetype::GoodVillager(GoodVillager::Hunter),
			VillagerArchetype::Outcast(Outcast::Wretch),
			VillagerArchetype::Minion(Minion::Witch),
		],
		DrawStats::new(4, 1, 1, 0),
		1,
		false,
	);

	run_game(
		&game_state,
		vec![
			TestAction::TryReveal(RevealResult::new(
				VillagerIndex::number(1),
				Some(VillagerInstance::new(
					VillagerArchetype::GoodVillager(GoodVillager::Medium),
					Some(Expression::Leaf(Testimony::Role(RoleClaim::new(
						VillagerIndex::number(2),
						VillagerArchetype::GoodVillager(GoodVillager::Hunter),
					)))),
				)),
			)),
			TestAction::TryReveal(RevealResult::new(
				VillagerIndex::number(2),
				Some(VillagerInstance::new(
					VillagerArchetype::GoodVillager(GoodVillager::Hunter),
					Some(Testimony::hunter(
						&VillagerIndex::number(2),
						1,
						game_state.total_villagers(),
					)),
				)),
			)),
			TestAction::TryReveal(RevealResult::new(
				VillagerIndex::number(3),
				Some(VillagerInstance::new(
					VillagerArchetype::Outcast(Outcast::Wretch),
					Some(Expression::Leaf(Testimony::FakeEvil(
						VillagerIndex::number(3),
					))),
				)),
			)),
			TestAction::TryReveal(RevealResult::new(
				VillagerIndex::number(4),
				Some(VillagerInstance::new(
					VillagerArchetype::GoodVillager(GoodVillager::Confessor),
					Some(Expression::Leaf(Testimony::Confess(ConfessorClaim::Good))),
				)),
			)),
			TestAction::TryReveal(RevealResult::new(
				VillagerIndex::number(5),
				Some(VillagerInstance::new(
					VillagerArchetype::GoodVillager(GoodVillager::Scout),
					Some(Expression::Leaf(Testimony::Scout(ScoutClaim::new(
						VillagerArchetype::Minion(Minion::Witch),
						3,
					)))),
				)),
			)),
			TestAction::TryReveal(RevealResult::new(VillagerIndex::number(6), None)),
			TestAction::TryExecute(KillAttempt::new(
				VillagerIndex::number(6),
				Some(KillResult::Unrevealed(UnrevealedKillData::new(
					VillagerInstance::new(
						VillagerArchetype::GoodVillager(GoodVillager::Confessor),
						Some(Expression::Leaf(Testimony::Confess(ConfessorClaim::Dizzy))),
					),
					KillData::new(Some(VillagerArchetype::Minion(Minion::Witch)), false).unwrap(),
				))),
			)),
		],
		None,
	);
}
