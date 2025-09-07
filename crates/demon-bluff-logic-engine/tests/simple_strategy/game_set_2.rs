use demon_bluff_gameplay_engine::{
	Expression,
	game_state::{
		AbilityResult, Action, DrawStats, KillAttempt, KillData, KillResult, RevealResult,
		UnrevealedKillData, new_game,
	},
	testimony::{ConfessorClaim, Direction, EvilPairsClaim, RoleClaim, ScoutClaim, Testimony},
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

// https://cdn.discordapp.com/attachments/1145879778457550850/1414286327389618276/image.png
// https://cdn.discordapp.com/attachments/1145879778457550850/1414286761621721191/image.png
#[test]
fn game_12() {
	let game_state = new_game(
		vec![
			VillagerArchetype::GoodVillager(GoodVillager::Empress),
			VillagerArchetype::GoodVillager(GoodVillager::Medium),
			VillagerArchetype::GoodVillager(GoodVillager::Gemcrafter),
			VillagerArchetype::GoodVillager(GoodVillager::Bard),
			VillagerArchetype::GoodVillager(GoodVillager::Lover),
			VillagerArchetype::Outcast(Outcast::Wretch),
			VillagerArchetype::Outcast(Outcast::Bombardier),
			VillagerArchetype::Minion(Minion::Poisoner),
			VillagerArchetype::Demon(Demon::Baa),
		],
		DrawStats::new(4, 1, 1, 1),
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
						VillagerIndex::number(3),
						VillagerIndex::number(5),
					])),
				)),
			)),
			TestAction::TryReveal(RevealResult::new(
				VillagerIndex::number(2),
				Some(VillagerInstance::new(
					VillagerArchetype::Outcast(Outcast::Wretch),
					Some(Expression::Leaf(Testimony::FakeEvil(
						VillagerIndex::number(2),
					))),
				)),
			)),
			TestAction::TryReveal(RevealResult::new(
				VillagerIndex::number(3),
				Some(VillagerInstance::new(
					VillagerArchetype::GoodVillager(GoodVillager::Gemcrafter),
					Some(Expression::Leaf(Testimony::Good(VillagerIndex::number(6)))),
				)),
			)),
			TestAction::TryReveal(RevealResult::new(
				VillagerIndex::number(4),
				Some(VillagerInstance::new(
					VillagerArchetype::GoodVillager(GoodVillager::Medium),
					Some(Expression::Leaf(Testimony::Role(RoleClaim::new(
						VillagerIndex::number(1),
						VillagerArchetype::GoodVillager(GoodVillager::Empress),
					)))),
				)),
			)),
			TestAction::TryReveal(RevealResult::new(
				VillagerIndex::number(5),
				Some(VillagerInstance::new(
					VillagerArchetype::GoodVillager(GoodVillager::Lover),
					Some(Testimony::lover(
						&VillagerIndex::number(5),
						2,
						game_state.total_villagers(),
					)),
				)),
			)),
			TestAction::TryReveal(RevealResult::new(
				VillagerIndex::number(6),
				Some(VillagerInstance::new(
					VillagerArchetype::GoodVillager(GoodVillager::Bard),
					Some(Expression::Leaf(Testimony::Bard(Some(3)))),
				)),
			)),
			TestAction::TryReveal(RevealResult::new(
				VillagerIndex::number(7),
				Some(VillagerInstance::new(
					VillagerArchetype::GoodVillager(GoodVillager::Lover),
					Some(Testimony::lover(
						&VillagerIndex::number(7),
						1,
						game_state.total_villagers(),
					)),
				)),
			)),
			TestAction::TryExecute(KillAttempt::new(
				VillagerIndex::number(6),
				Some(KillResult::Revealed(
					KillData::new(None, true).expect("Bad kill data?"),
				)),
			)),
			TestAction::TryExecute(KillAttempt::new(
				VillagerIndex::number(5),
				Some(KillResult::Revealed(
					KillData::new(Some(VillagerArchetype::Minion(Minion::Poisoner)), false)
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
		Some(7),
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
	// TODO
}

#[test]
fn game_15() {
	// TODO
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

#[test]
fn game_17() {
	let game_state = new_game(
		vec![
			VillagerArchetype::GoodVillager(GoodVillager::Hunter),
			VillagerArchetype::GoodVillager(GoodVillager::Scout),
			VillagerArchetype::GoodVillager(GoodVillager::Knitter),
			VillagerArchetype::GoodVillager(GoodVillager::Enlightened),
			VillagerArchetype::GoodVillager(GoodVillager::Knight),
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
					VillagerArchetype::GoodVillager(GoodVillager::Enlightened),
					Some(Testimony::englightened(
						&VillagerIndex(1),
						Direction::CounterClockwise,
						game_state.total_villagers(),
					)),
				)),
			)),
			TestAction::TryReveal(RevealResult::new(
				VillagerIndex::number(2),
				Some(VillagerInstance::new(
					VillagerArchetype::GoodVillager(GoodVillager::Scout),
					Some(Expression::Leaf(Testimony::Scout(ScoutClaim::new(
						VillagerArchetype::Minion(Minion::Witch),
						1,
					)))),
				)),
			)),
			TestAction::TryReveal(RevealResult::new(
				VillagerIndex::number(3),
				Some(VillagerInstance::new(
					VillagerArchetype::GoodVillager(GoodVillager::Knitter),
					Some(Expression::Leaf(Testimony::Knitter(EvilPairsClaim::new(1)))),
				)),
			)),
			TestAction::TryReveal(RevealResult::new(
				VillagerIndex::number(4),
				Some(VillagerInstance::new(
					VillagerArchetype::Outcast(Outcast::Wretch),
					Some(Expression::Leaf(Testimony::FakeEvil(
						VillagerIndex::number(4),
					))),
				)),
			)),
			TestAction::TryReveal(RevealResult::new(
				VillagerIndex::number(5),
				Some(VillagerInstance::new(
					VillagerArchetype::GoodVillager(GoodVillager::Hunter),
					Some(Testimony::hunter(
						&VillagerIndex::number(5),
						3,
						game_state.total_villagers(),
					)),
				)),
			)),
			TestAction::TryExecute(KillAttempt::new(
				VillagerIndex::number(5),
				Some(KillResult::Revealed(
					KillData::new(Some(VillagerArchetype::Minion(Minion::Witch)), false).unwrap(),
				)),
			)),
		],
		None,
	);
}
