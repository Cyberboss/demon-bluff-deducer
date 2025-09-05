use demon_bluff_gameplay_engine::{
	Expression,
	game_state::{Action, DrawStats, KillAttempt, KillData, KillResult, RevealResult, new_game},
	testimony::{ConfessorClaim, Direction, RoleClaim, Testimony},
	villager::{GoodVillager, Minion, Outcast, VillagerArchetype, VillagerIndex, VillagerInstance},
};
use test_helpers::run_game;

mod test_helpers;

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
			Action::TryReveal(RevealResult::new(
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
			Action::TryReveal(RevealResult::new(
				VillagerIndex(1),
				Some(VillagerInstance::new(
					VillagerArchetype::GoodVillager(GoodVillager::Medium),
					Some(Expression::Leaf(Testimony::Role(RoleClaim::new(
						VillagerIndex(2),
						VillagerArchetype::GoodVillager(GoodVillager::Hunter),
					)))),
				)),
			)),
			Action::TryReveal(RevealResult::new(
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
			Action::TryExecute(KillAttempt::new(
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
			Action::TryReveal(RevealResult::new(
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
			Action::TryReveal(RevealResult::new(
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
			Action::TryReveal(RevealResult::new(
				VillagerIndex::number(3),
				Some(VillagerInstance::new(
					VillagerArchetype::Outcast(Outcast::Wretch),
					Some(Expression::Leaf(Testimony::FakeEvil(
						VillagerIndex::number(3),
					))),
				)),
			)),
			Action::TryReveal(RevealResult::new(
				VillagerIndex::number(4),
				Some(VillagerInstance::new(
					VillagerArchetype::GoodVillager(GoodVillager::Medium),
					Some(Expression::Leaf(Testimony::Role(RoleClaim::new(
						VillagerIndex::number(7),
						VillagerArchetype::GoodVillager(GoodVillager::Confessor),
					)))),
				)),
			)),
			Action::TryReveal(RevealResult::new(
				VillagerIndex::number(5),
				Some(VillagerInstance::new(
					VillagerArchetype::GoodVillager(GoodVillager::Medium),
					Some(Expression::Leaf(Testimony::Role(RoleClaim::new(
						VillagerIndex::number(6),
						VillagerArchetype::GoodVillager(GoodVillager::Jester),
					)))),
				)),
			)),
			Action::TryReveal(RevealResult::new(
				VillagerIndex::number(6),
				Some(VillagerInstance::new(
					VillagerArchetype::GoodVillager(GoodVillager::Jester),
					None,
				)),
			)),
			Action::TryExecute(KillAttempt::new(
				VillagerIndex::number(6),
				Some(KillResult::Revealed(
					KillData::new(Some(VillagerArchetype::Minion(Minion::Twinion)), false)
						.expect("Bad kill data?"),
				)),
			)),
			Action::TryExecute(KillAttempt::new(
				VillagerIndex::number(5),
				Some(KillResult::Revealed(
					KillData::new(Some(VillagerArchetype::Minion(Minion::Minion)), false)
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
			Action::TryReveal(RevealResult::new(
				VillagerIndex::number(1),
				Some(VillagerInstance::new(
					VillagerArchetype::GoodVillager(GoodVillager::Knight),
					Some(Expression::Leaf(Testimony::Invincible(
						VillagerIndex::number(1),
					))),
				)),
			)),
			Action::TryReveal(RevealResult::new(
				VillagerIndex::number(2),
				Some(VillagerInstance::new(
					VillagerArchetype::GoodVillager(GoodVillager::Medium),
					Some(Expression::Leaf(Testimony::Role(RoleClaim::new(
						VillagerIndex::number(3),
						VillagerArchetype::GoodVillager(GoodVillager::Hunter),
					)))),
				)),
			)),
			Action::TryReveal(RevealResult::new(
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
			Action::TryReveal(RevealResult::new(
				VillagerIndex::number(4),
				Some(VillagerInstance::new(
					VillagerArchetype::GoodVillager(GoodVillager::Jester),
					None,
				)),
			)),
			Action::TryReveal(RevealResult::new(
				VillagerIndex::number(5),
				Some(VillagerInstance::new(
					VillagerArchetype::GoodVillager(GoodVillager::Gemcrafter),
					Some(Expression::Leaf(Testimony::Good(VillagerIndex::number(8)))),
				)),
			)),
			Action::TryReveal(RevealResult::new(
				VillagerIndex::number(6),
				Some(VillagerInstance::new(
					VillagerArchetype::GoodVillager(GoodVillager::Medium),
					Some(Expression::Leaf(Testimony::Role(RoleClaim::new(
						VillagerIndex::number(5),
						VillagerArchetype::GoodVillager(GoodVillager::Gemcrafter),
					)))),
				)),
			)),
			Action::TryExecute(KillAttempt::new(
				VillagerIndex::number(2),
				Some(KillResult::Revealed(
					KillData::new(Some(VillagerArchetype::Minion(Minion::Minion)), false)
						.expect("Bad kill data?"),
				)),
			)),
			Action::TryExecute(KillAttempt::new(
				VillagerIndex::number(3),
				Some(KillResult::Revealed(
					KillData::new(Some(VillagerArchetype::Minion(Minion::Twinion)), false)
						.expect("Bad kill data?"),
				)),
			)),
		],
		Some(7),
	);
}
