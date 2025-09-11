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

// https://cdn.discordapp.com/attachments/1145879778457550850/1415492973050990652/image.png
// https://cdn.discordapp.com/attachments/1145879778457550850/1415496362002354247/image.png
#[test]
fn game_21() {
	let game_state = new_game(
		vec![
			VillagerArchetype::GoodVillager(GoodVillager::Empress),
			VillagerArchetype::GoodVillager(GoodVillager::FortuneTeller),
			VillagerArchetype::GoodVillager(GoodVillager::Bard),
			VillagerArchetype::GoodVillager(GoodVillager::Knitter),
			VillagerArchetype::GoodVillager(GoodVillager::Gemcrafter),
			VillagerArchetype::GoodVillager(GoodVillager::Alchemist),
			VillagerArchetype::GoodVillager(GoodVillager::Lover),
			VillagerArchetype::Outcast(Outcast::Doppelganger),
			VillagerArchetype::Outcast(Outcast::PlagueDoctor),
			VillagerArchetype::Outcast(Outcast::Bombardier),
			VillagerArchetype::Minion(Minion::Minion),
			VillagerArchetype::Minion(Minion::Counsellor),
			VillagerArchetype::Demon(Demon::Baa),
		],
		DrawStats::new(5, 1, 2, 1),
		3,
		false,
	);

	run_game(
		&game_state,
		vec![
			TestAction::TryReveal(RevealResult::new(
				VillagerIndex::number(1),
				Some(VillagerInstance::new(
					VillagerArchetype::GoodVillager(GoodVillager::Bard),
					Some(Expression::Leaf(Testimony::Bard(None))),
				)),
			)),
			TestAction::TryReveal(RevealResult::new(
				VillagerIndex::number(2),
				Some(VillagerInstance::new(
					VillagerArchetype::GoodVillager(GoodVillager::Gemcrafter),
					Some(Expression::Leaf(Testimony::Good(VillagerIndex::number(5)))),
				)),
			)),
			TestAction::TryReveal(RevealResult::new(
				VillagerIndex::number(3),
				Some(VillagerInstance::new(
					VillagerArchetype::GoodVillager(GoodVillager::FortuneTeller),
					None,
				)),
			)),
			TestAction::TryReveal(RevealResult::new(
				VillagerIndex::number(4),
				Some(VillagerInstance::new(
					VillagerArchetype::GoodVillager(GoodVillager::Gemcrafter),
					Some(Expression::Leaf(Testimony::Good(VillagerIndex::number(6)))),
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
					VillagerArchetype::GoodVillager(GoodVillager::Knitter),
					Some(Expression::Leaf(Testimony::Knitter(EvilPairsClaim::new(2)))),
				)),
			)),
			TestAction::TryReveal(RevealResult::new(
				VillagerIndex::number(7),
				Some(VillagerInstance::new(
					VillagerArchetype::Outcast(Outcast::Bombardier),
					Some(Expression::Leaf(Testimony::SelfDestruct(
						VillagerIndex::number(7),
					))),
				)),
			)),
			TestAction::TryReveal(RevealResult::new(
				VillagerIndex::number(8),
				Some(VillagerInstance::new(
					VillagerArchetype::GoodVillager(GoodVillager::Empress),
					Some(Testimony::empress(&[
						VillagerIndex::number(2),
						VillagerIndex::number(5),
						VillagerIndex::number(1),
					])),
				)),
			)),
			TestAction::TryReveal(RevealResult::new(
				VillagerIndex::number(9),
				Some(VillagerInstance::new(
					VillagerArchetype::GoodVillager(GoodVillager::Lover),
					Some(Testimony::lover(
						&VillagerIndex::number(9),
						1,
						game_state.total_villagers(),
					)),
				)),
			)),
			TestAction::TryExecute(KillAttempt::new(
				VillagerIndex::number(8),
				Some(KillResult::Revealed(
					KillData::new(Some(VillagerArchetype::Demon(Demon::Baa)), false)
						.expect("Bad kill data?"),
				)),
			)),
			TestAction::TryExecute(KillAttempt::new(
				VillagerIndex::number(6),
				Some(KillResult::Revealed(
					KillData::new(Some(VillagerArchetype::Minion(Minion::Counsellor)), false)
						.expect("Bad kill data?"),
				)),
			)),
			TestAction::TryExecute(KillAttempt::new(
				VillagerIndex::number(4),
				Some(KillResult::Revealed(
					KillData::new(Some(VillagerArchetype::Minion(Minion::Minion)), false)
						.expect("Bad kill data?"),
				)),
			)),
		],
		None,
	);
}
