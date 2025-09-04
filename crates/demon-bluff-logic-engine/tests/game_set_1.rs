use demon_bluff_gameplay_engine::{
	Expression,
	game_state::{Action, DrawStats, KillAttempt, KillData, KillResult, RevealResult, new_game},
	testimony::{RoleClaim, Testimony},
	villager::{GoodVillager, Minion, VillagerArchetype, VillagerIndex, VillagerInstance},
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
		Some(3),
	);
}
