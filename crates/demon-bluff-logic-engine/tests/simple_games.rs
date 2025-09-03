use demon_bluff_gameplay_engine::{
	Expression,
	game_state::{Action, DrawStats, KillAttempt, KillData, KillResult, RevealResult, new_game},
	testimony::{ConfessorClaim, Direction, Testimony},
	villager::{GoodVillager, Minion, VillagerArchetype, VillagerIndex, VillagerInstance},
};
use test_helpers::run_game;

mod test_helpers;

#[test]
fn simple_game_4() {
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
			Action::TryReveal(RevealResult::new(
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
			Action::TryReveal(RevealResult::new(
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
			Action::TryReveal(RevealResult::new(
				VillagerIndex(2),
				Some(VillagerInstance::new(
					VillagerArchetype::GoodVillager(GoodVillager::Gemcrafter),
					Some(Expression::Leaf(Testimony::Good(VillagerIndex(2)))),
				)),
			)),
			Action::TryReveal(RevealResult::new(
				VillagerIndex(3),
				Some(VillagerInstance::new(
					VillagerArchetype::GoodVillager(GoodVillager::Confessor),
					Some(Expression::Leaf(Testimony::Confess(ConfessorClaim::Good))),
				)),
			)),
			Action::TryReveal(RevealResult::new(
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
			Action::TryExecute(KillAttempt::new(
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
