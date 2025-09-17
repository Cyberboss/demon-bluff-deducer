use std::num::NonZeroUsize;

use demon_bluff_gameplay_engine::{
	Expression,
	game_state::{
		AbilityResult, Action, DrawStats, KillAttempt, KillData, KillResult, RevealResult,
		SlayerKill, UnrevealedKillData, new_game,
	},
	testimony::{
		ArchitectClaim, BakerClaim, BishopClaim, BishopEvil, ConfessorClaim, Direction,
		DreamerClaim, DruidClaim, EvilPairsClaim, RoleClaim, ScoutClaim, SlayResult, Testimony,
	},
	villager::{
		Demon, GoodVillager, Minion, Outcast, VillagerArchetype, VillagerIndex, VillagerInstance,
	},
};

use super::run_game;
use crate::{helpers::TestAction, simple_strategy::run_game_ack_unsolvable};

// https://cdn.discordapp.com/attachments/487268744419344384/1417689458253234176/image.png
// https://cdn.discordapp.com/attachments/487268744419344384/1417695190604320808/image.png
#[test]
fn game_0031() {
	// TODO: This test is SLOW profile it!
	let game_state = new_game(
		vec![
			VillagerArchetype::GoodVillager(GoodVillager::Bard),
			VillagerArchetype::GoodVillager(GoodVillager::Medium),
			VillagerArchetype::GoodVillager(GoodVillager::Knight),
			VillagerArchetype::GoodVillager(GoodVillager::Dreamer),
			VillagerArchetype::GoodVillager(GoodVillager::Architect),
			VillagerArchetype::GoodVillager(GoodVillager::Enlightened),
			VillagerArchetype::GoodVillager(GoodVillager::Empress),
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
					VillagerArchetype::GoodVillager(GoodVillager::Empress),
					Some(Testimony::empress(&[
						VillagerIndex::number(9),
						VillagerIndex::number(2),
						VillagerIndex::number(4),
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
					VillagerArchetype::GoodVillager(GoodVillager::Enlightened),
					Some(Expression::Leaf(Testimony::Enlightened(
						Direction::Clockwise,
					))),
				)),
			)),
			TestAction::TryReveal(RevealResult::new(
				VillagerIndex::number(4),
				Some(VillagerInstance::new(
					VillagerArchetype::GoodVillager(GoodVillager::Empress),
					Some(Testimony::empress(&[
						VillagerIndex::number(9),
						VillagerIndex::number(7),
						VillagerIndex::number(2),
					])),
				)),
			)),
			TestAction::TryReveal(RevealResult::new(
				VillagerIndex::number(5),
				Some(VillagerInstance::new(
					VillagerArchetype::GoodVillager(GoodVillager::Enlightened),
					Some(Expression::Leaf(Testimony::Enlightened(
						Direction::Equidistant,
					))),
				)),
			)),
			TestAction::TryReveal(RevealResult::new(
				VillagerIndex::number(6),
				Some(VillagerInstance::new(
					VillagerArchetype::GoodVillager(GoodVillager::Dreamer),
					None,
				)),
			)),
			TestAction::TryReveal(RevealResult::new(
				VillagerIndex::number(7),
				Some(VillagerInstance::new(
					VillagerArchetype::GoodVillager(GoodVillager::Bard),
					Some(Expression::Leaf(Testimony::Bard(Some(
						NonZeroUsize::new(4).unwrap(),
					)))),
				)),
			)),
			TestAction::TryReveal(RevealResult::new(
				VillagerIndex::number(8),
				Some(VillagerInstance::new(
					VillagerArchetype::GoodVillager(GoodVillager::Architect),
					Some(Expression::Leaf(Testimony::Architect(
						ArchitectClaim::Right,
					))),
				)),
			)),
			TestAction::TryReveal(RevealResult::new(
				VillagerIndex::number(9),
				Some(VillagerInstance::new(
					VillagerArchetype::Outcast(Outcast::PlagueDoctor),
					None,
				)),
			)),
			TestAction::Ability(
				vec![VillagerIndex::number(7)],
				AbilityResult::new(
					VillagerIndex::number(6),
					Some(Expression::Leaf(Testimony::Dreamer(DreamerClaim::new(
						VillagerIndex::number(7),
						Some(VillagerArchetype::Demon(Demon::Baa)),
					)))),
					None,
				),
			),
			TestAction::Ability(
				vec![VillagerIndex::number(7)],
				AbilityResult::new(
					VillagerIndex::number(9),
					Some(Expression::Not(Box::new(Expression::Leaf(
						Testimony::Corrupt(VillagerIndex::number(7)),
					)))),
					None,
				),
			),
			TestAction::TryExecute(KillAttempt::new(
				VillagerIndex::number(7),
				Some(KillResult::Revealed(
					KillData::new(None, false).expect("Bad kill data?"),
				)),
			)),
			TestAction::TryExecute(KillAttempt::new(
				VillagerIndex::number(6),
				Some(KillResult::Revealed(
					KillData::new(Some(VillagerArchetype::Demon(Demon::Baa)), false)
						.expect("Bad kill data?"),
				)),
			)),
			TestAction::TryExecute(KillAttempt::new(
				VillagerIndex::number(8),
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
