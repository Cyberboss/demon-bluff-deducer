use std::num::NonZeroUsize;

use demon_bluff_gameplay_engine::{
	Expression,
	game_state::{
		AbilityResult, Action, DrawStats, KillAttempt, KillData, KillResult, RevealResult,
		SlayerKill, UnrevealedKillData, new_game,
	},
	testimony::{
		ArchitectClaim, BishopClaim, BishopEvil, ConfessorClaim, Direction, DruidClaim,
		EvilPairsClaim, RoleClaim, ScoutClaim, Testimony,
	},
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
	// TODO: This test is SLOW profile it!
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

// https://cdn.discordapp.com/attachments/1145879778457550850/1415498314673160292/image.png
// https://cdn.discordapp.com/attachments/1145879778457550850/1415501038366101535/image.png
// This game didn't seem solvable...
#[test]
fn game_22() {
	let game_state = new_game(
		vec![
			VillagerArchetype::GoodVillager(GoodVillager::Knight),
			VillagerArchetype::GoodVillager(GoodVillager::FortuneTeller),
			VillagerArchetype::GoodVillager(GoodVillager::Lover),
			VillagerArchetype::GoodVillager(GoodVillager::Bard),
			VillagerArchetype::GoodVillager(GoodVillager::Scout),
			VillagerArchetype::GoodVillager(GoodVillager::Oracle),
			VillagerArchetype::GoodVillager(GoodVillager::Slayer),
			VillagerArchetype::Outcast(Outcast::PlagueDoctor),
			VillagerArchetype::Minion(Minion::Witch),
			VillagerArchetype::Demon(Demon::Pooka),
		],
		DrawStats::new(6, 1, 1, 1),
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
					VillagerArchetype::GoodVillager(GoodVillager::Lover),
					Some(Testimony::lover(
						&VillagerIndex::number(2),
						0,
						game_state.total_villagers(),
					)),
				)),
			)),
			TestAction::TryReveal(RevealResult::new(
				VillagerIndex::number(3),
				Some(VillagerInstance::new(
					VillagerArchetype::GoodVillager(GoodVillager::Scout),
					Some(Expression::Leaf(Testimony::Scout(ScoutClaim::new(
						VillagerArchetype::Demon(Demon::Pooka),
						2,
					)))),
				)),
			)),
			TestAction::TryReveal(RevealResult::new(
				VillagerIndex::number(4),
				Some(VillagerInstance::new(
					VillagerArchetype::GoodVillager(GoodVillager::Slayer),
					None,
				)),
			)),
			TestAction::TryReveal(RevealResult::new(
				VillagerIndex::number(5),
				Some(VillagerInstance::new(
					VillagerArchetype::GoodVillager(GoodVillager::Oracle),
					Some(Testimony::oracle(
						&[VillagerIndex::number(1), VillagerIndex::number(2)],
						VillagerArchetype::Minion(Minion::Witch),
					)),
				)),
			)),
			TestAction::TryReveal(RevealResult::new(
				VillagerIndex::number(6),
				Some(VillagerInstance::new(
					VillagerArchetype::GoodVillager(GoodVillager::Bard),
					Some(Expression::Leaf(Testimony::Bard(Some(
						NonZeroUsize::new(1).unwrap(),
					)))),
				)),
			)),
			TestAction::TryReveal(RevealResult::new(
				VillagerIndex::number(7),
				Some(VillagerInstance::new(
					VillagerArchetype::GoodVillager(GoodVillager::Knight),
					Some(Expression::Leaf(Testimony::Invincible(
						VillagerIndex::number(7),
					))),
				)),
			)),
			TestAction::TryReveal(RevealResult::new(
				VillagerIndex::number(8),
				Some(VillagerInstance::new(
					VillagerArchetype::Outcast(Outcast::PlagueDoctor),
					None,
				)),
			)),
			TestAction::TryReveal(RevealResult::new(VillagerIndex::number(9), None)),
			TestAction::Ability(
				vec![VillagerIndex::number(2)],
				AbilityResult::new(
					VillagerIndex::number(4),
					Some(Testimony::slayer(
						VillagerIndex::number(4),
						VillagerIndex::number(2),
						false,
					)),
					None,
				),
			),
			TestAction::TryExecute(KillAttempt::new(
				VillagerIndex::number(7),
				Some(KillResult::Revealed(
					KillData::new(Some(VillagerArchetype::Minion(Minion::Witch)), false)
						.expect("Bad kill data?"),
				)),
			)),
			TestAction::TryReveal(RevealResult::new(
				VillagerIndex::number(9),
				Some(VillagerInstance::new(
					VillagerArchetype::GoodVillager(GoodVillager::FortuneTeller),
					None,
				)),
			)),
			TestAction::Ability(
				vec![VillagerIndex::number(6)],
				AbilityResult::new(
					VillagerIndex::number(8),
					Some(Expression::Not(Box::new(Expression::Leaf(
						Testimony::Corrupt(VillagerIndex::number(6)),
					)))),
					None,
				),
			),
			TestAction::Ability(
				vec![VillagerIndex::number(9), VillagerIndex::number(7)],
				AbilityResult::new(
					VillagerIndex::number(9),
					Some(Testimony::fortune_teller(
						&[VillagerIndex::number(9), VillagerIndex::number(7)],
						true,
					)),
					None,
				),
			),
			TestAction::TryExecute(KillAttempt::new(
				VillagerIndex::number(3),
				Some(KillResult::Revealed(
					KillData::new(None, false).expect("Bad kill data?"),
				)),
			)),
			TestAction::TryExecute(KillAttempt::new(
				VillagerIndex::number(4),
				Some(KillResult::Revealed(
					KillData::new(Some(VillagerArchetype::Demon(Demon::Pooka)), false)
						.expect("Bad kill data?"),
				)),
			)),
		],
		None,
	);
}

// https://cdn.discordapp.com/attachments/487268744419344384/1416926552557359205/image.png
// https://cdn.discordapp.com/attachments/487268744419344384/1416928394201010321/image.png
#[test]
fn game_23() {
	let game_state = new_game(
		vec![
			VillagerArchetype::GoodVillager(GoodVillager::Enlightened),
			VillagerArchetype::GoodVillager(GoodVillager::Knight),
			VillagerArchetype::GoodVillager(GoodVillager::Alchemist),
			VillagerArchetype::GoodVillager(GoodVillager::Slayer),
			VillagerArchetype::GoodVillager(GoodVillager::Empress),
			VillagerArchetype::GoodVillager(GoodVillager::Medium),
			VillagerArchetype::GoodVillager(GoodVillager::Hunter),
			VillagerArchetype::Outcast(Outcast::PlagueDoctor),
			VillagerArchetype::Outcast(Outcast::Bombardier),
			VillagerArchetype::Minion(Minion::Witch),
			VillagerArchetype::Demon(Demon::Pooka),
		],
		DrawStats::new(6, 1, 1, 1),
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
					VillagerArchetype::GoodVillager(GoodVillager::Alchemist),
					Some(Testimony::alchemist(
						&VillagerIndex::number(2),
						2,
						game_state.total_villagers(),
					)),
				)),
			)),
			TestAction::TryReveal(RevealResult::new(
				VillagerIndex::number(3),
				Some(VillagerInstance::new(
					VillagerArchetype::GoodVillager(GoodVillager::Empress),
					Some(Testimony::empress(&[
						VillagerIndex::number(7),
						VillagerIndex::number(4),
						VillagerIndex::number(1),
					])),
				)),
			)),
			TestAction::TryReveal(RevealResult::new(
				VillagerIndex::number(4),
				Some(VillagerInstance::new(
					VillagerArchetype::GoodVillager(GoodVillager::Medium),
					Some(Expression::Leaf(Testimony::Role(RoleClaim::new(
						VillagerIndex::number(6),
						VillagerArchetype::GoodVillager(GoodVillager::Enlightened),
					)))),
				)),
			)),
			TestAction::TryReveal(RevealResult::new(
				VillagerIndex::number(5),
				Some(VillagerInstance::new(
					VillagerArchetype::Outcast(Outcast::Bombardier),
					Some(Expression::Leaf(Testimony::SelfDestruct(
						VillagerIndex::number(5),
					))),
				)),
			)),
			TestAction::TryReveal(RevealResult::new(
				VillagerIndex::number(6),
				Some(VillagerInstance::new(
					VillagerArchetype::GoodVillager(GoodVillager::Enlightened),
					Some(Expression::Leaf(Testimony::Enlightened(
						Direction::Clockwise,
					))),
				)),
			)),
			TestAction::TryReveal(RevealResult::new(
				VillagerIndex::number(7),
				Some(VillagerInstance::new(
					VillagerArchetype::GoodVillager(GoodVillager::Hunter),
					Some(Testimony::hunter(
						&VillagerIndex::number(7),
						2,
						game_state.total_villagers(),
					)),
				)),
			)),
			TestAction::TryReveal(RevealResult::new(
				VillagerIndex::number(8),
				Some(VillagerInstance::new(
					VillagerArchetype::Outcast(Outcast::PlagueDoctor),
					None,
				)),
			)),
			TestAction::TryExecute(KillAttempt::new(
				VillagerIndex::number(5),
				Some(KillResult::Revealed(
					KillData::new(Some(VillagerArchetype::Minion(Minion::Witch)), false)
						.expect("Bad kill data?"),
				)),
			)),
			TestAction::TryReveal(RevealResult::new(
				VillagerIndex::number(9),
				Some(VillagerInstance::new(
					VillagerArchetype::GoodVillager(GoodVillager::Slayer),
					None,
				)),
			)),
			TestAction::Ability(
				vec![VillagerIndex::number(1)],
				AbilityResult::new(
					VillagerIndex::number(8),
					Some(Expression::Not(Box::new(Expression::Leaf(
						Testimony::Corrupt(VillagerIndex::number(1)),
					)))),
					None,
				),
			),
			TestAction::TryExecute(KillAttempt::new(
				VillagerIndex::number(1),
				Some(KillResult::Revealed(
					KillData::new(Some(VillagerArchetype::Demon(Demon::Pooka)), false)
						.expect("Bad kill data?"),
				)),
			)),
		],
		Some(10),
	);
}

// https://cdn.discordapp.com/attachments/487268744419344384/1416931826273878057/image.png
// https://cdn.discordapp.com/attachments/487268744419344384/1416941370731532401/image.png
#[test]
fn game_24() {
	let game_state = new_game(
		vec![
			VillagerArchetype::GoodVillager(GoodVillager::Knight),
			VillagerArchetype::GoodVillager(GoodVillager::Bard),
			VillagerArchetype::GoodVillager(GoodVillager::Scout),
			VillagerArchetype::GoodVillager(GoodVillager::Oracle),
			VillagerArchetype::GoodVillager(GoodVillager::Druid),
			VillagerArchetype::GoodVillager(GoodVillager::Confessor),
			VillagerArchetype::Outcast(Outcast::Drunk),
			VillagerArchetype::Outcast(Outcast::Wretch),
			VillagerArchetype::Outcast(Outcast::Bombardier),
			VillagerArchetype::Minion(Minion::Counsellor),
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
					VillagerArchetype::Outcast(Outcast::Bombardier),
					Some(Expression::Leaf(Testimony::SelfDestruct(
						VillagerIndex::number(1),
					))),
				)),
			)),
			TestAction::TryReveal(RevealResult::new(
				VillagerIndex::number(2),
				Some(VillagerInstance::new(
					VillagerArchetype::GoodVillager(GoodVillager::Confessor),
					Some(Expression::Leaf(Testimony::Confess(ConfessorClaim::Good))),
				)),
			)),
			TestAction::TryReveal(RevealResult::new(
				VillagerIndex::number(3),
				Some(VillagerInstance::new(
					VillagerArchetype::GoodVillager(GoodVillager::Knight),
					Some(Expression::Leaf(Testimony::Invincible(
						VillagerIndex::number(3),
					))),
				)),
			)),
			TestAction::TryReveal(RevealResult::new(
				VillagerIndex::number(4),
				Some(VillagerInstance::new(
					VillagerArchetype::GoodVillager(GoodVillager::Druid),
					None,
				)),
			)),
			TestAction::TryReveal(RevealResult::new(
				VillagerIndex::number(5),
				Some(VillagerInstance::new(
					VillagerArchetype::GoodVillager(GoodVillager::Scout),
					Some(Expression::Leaf(Testimony::Scout(ScoutClaim::new(
						VillagerArchetype::Demon(Demon::Baa),
						3,
					)))),
				)),
			)),
			TestAction::TryReveal(RevealResult::new(
				VillagerIndex::number(6),
				Some(VillagerInstance::new(
					VillagerArchetype::GoodVillager(GoodVillager::Bard),
					Some(Expression::Leaf(Testimony::Bard(Some(
						NonZeroUsize::new(2).unwrap(),
					)))),
				)),
			)),
			TestAction::TryReveal(RevealResult::new(
				VillagerIndex::number(7),
				Some(VillagerInstance::new(
					VillagerArchetype::GoodVillager(GoodVillager::Knight),
					Some(Expression::Leaf(Testimony::Invincible(
						VillagerIndex::number(7),
					))),
				)),
			)),
			TestAction::Ability(
				vec![
					VillagerIndex::number(1),
					VillagerIndex::number(3),
					VillagerIndex::number(7),
				],
				AbilityResult::new(
					VillagerIndex::number(4),
					Some(Expression::Leaf(Testimony::Druid(DruidClaim::new(
						&[
							VillagerIndex::number(1),
							VillagerIndex::number(3),
							VillagerIndex::number(7),
						],
						None,
					)))),
					None,
				),
			),
			TestAction::TryExecute(KillAttempt::new(
				VillagerIndex::number(7),
				Some(KillResult::Revealed(
					KillData::new(Some(VillagerArchetype::Demon(Demon::Baa)), false)
						.expect("Bad kill data?"),
				)),
			)),
			TestAction::TryExecute(KillAttempt::new(
				VillagerIndex::number(3),
				Some(KillResult::Revealed(
					KillData::new(Some(VillagerArchetype::Minion(Minion::Counsellor)), false)
						.expect("Bad kill data?"),
				)),
			)),
		],
		None,
	);
}

// https://cdn.discordapp.com/attachments/487268744419344384/1416944187521110118/image.png
// https://cdn.discordapp.com/attachments/487268744419344384/1416948590609170563/image.png
#[test]
fn game_25() {
	let game_state = new_game(
		vec![
			VillagerArchetype::GoodVillager(GoodVillager::Medium),
			VillagerArchetype::GoodVillager(GoodVillager::Slayer),
			VillagerArchetype::GoodVillager(GoodVillager::Bishop),
			VillagerArchetype::GoodVillager(GoodVillager::Scout),
			VillagerArchetype::GoodVillager(GoodVillager::Alchemist),
			VillagerArchetype::GoodVillager(GoodVillager::Architect),
			VillagerArchetype::Outcast(Outcast::Bombardier),
			VillagerArchetype::Outcast(Outcast::PlagueDoctor),
			VillagerArchetype::Minion(Minion::Shaman),
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
					VillagerArchetype::GoodVillager(GoodVillager::Medium),
					Some(Expression::Leaf(Testimony::Role(RoleClaim::new(
						VillagerIndex::number(6),
						VillagerArchetype::GoodVillager(GoodVillager::Architect),
					)))),
				)),
			)),
			TestAction::TryReveal(RevealResult::new(
				VillagerIndex::number(2),
				Some(VillagerInstance::new(
					VillagerArchetype::GoodVillager(GoodVillager::Alchemist),
					Some(Testimony::alchemist(
						&VillagerIndex::number(2),
						0,
						game_state.total_villagers(),
					)),
				)),
			)),
			TestAction::TryReveal(RevealResult::new(
				VillagerIndex::number(3),
				Some(VillagerInstance::new(
					VillagerArchetype::GoodVillager(GoodVillager::Slayer),
					None,
				)),
			)),
			TestAction::TryReveal(RevealResult::new(
				VillagerIndex::number(4),
				Some(VillagerInstance::new(
					VillagerArchetype::GoodVillager(GoodVillager::Alchemist),
					Some(Testimony::alchemist(
						&VillagerIndex::number(4),
						0,
						game_state.total_villagers(),
					)),
				)),
			)),
			TestAction::TryReveal(RevealResult::new(
				VillagerIndex::number(5),
				Some(VillagerInstance::new(
					VillagerArchetype::Outcast(Outcast::Bombardier),
					Some(Expression::Leaf(Testimony::SelfDestruct(
						VillagerIndex::number(5),
					))),
				)),
			)),
			TestAction::TryReveal(RevealResult::new(
				VillagerIndex::number(6),
				Some(VillagerInstance::new(
					VillagerArchetype::GoodVillager(GoodVillager::Architect),
					Some(Expression::Leaf(Testimony::Architect(ArchitectClaim::Left))),
				)),
			)),
			TestAction::TryReveal(RevealResult::new(
				VillagerIndex::number(7),
				Some(VillagerInstance::new(
					VillagerArchetype::GoodVillager(GoodVillager::Scout),
					Some(Expression::Leaf(Testimony::Scout(ScoutClaim::new(
						VillagerArchetype::Demon(Demon::Baa),
						3,
					)))),
				)),
			)),
			TestAction::TryReveal(RevealResult::new(
				VillagerIndex::number(8),
				Some(VillagerInstance::new(
					VillagerArchetype::GoodVillager(GoodVillager::Medium),
					Some(Expression::Leaf(Testimony::Role(RoleClaim::new(
						VillagerIndex::number(5),
						VillagerArchetype::Outcast(Outcast::Bombardier),
					)))),
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
				VillagerIndex::number(1),
				Some(KillResult::Revealed(
					KillData::new(Some(VillagerArchetype::Minion(Minion::Shaman)), false)
						.expect("Bad kill data?"),
				)),
			)),
		],
		None,
	);
}

// https://cdn.discordapp.com/attachments/487268744419344384/1416949095666421830/image.png
// https://cdn.discordapp.com/attachments/487268744419344384/1416956112598601748/image.png
#[test]
fn game_26() {
	let game_state = new_game(
		vec![
			VillagerArchetype::GoodVillager(GoodVillager::Knitter),
			VillagerArchetype::GoodVillager(GoodVillager::Confessor),
			VillagerArchetype::GoodVillager(GoodVillager::Bishop),
			VillagerArchetype::GoodVillager(GoodVillager::Bard),
			VillagerArchetype::GoodVillager(GoodVillager::Knight),
			VillagerArchetype::GoodVillager(GoodVillager::Oracle),
			VillagerArchetype::GoodVillager(GoodVillager::Lover),
			VillagerArchetype::Outcast(Outcast::PlagueDoctor),
			VillagerArchetype::Outcast(Outcast::Drunk),
			VillagerArchetype::Minion(Minion::Puppeteer),
			VillagerArchetype::Demon(Demon::Pooka),
		],
		DrawStats::new(5, 1, 1, 1),
		3,
		false,
	);

	run_game(
		&game_state,
		vec![
			TestAction::TryReveal(RevealResult::new(
				VillagerIndex::number(1),
				Some(VillagerInstance::new(
					VillagerArchetype::Outcast(Outcast::PlagueDoctor),
					None,
				)),
			)),
			TestAction::TryReveal(RevealResult::new(
				VillagerIndex::number(2),
				Some(VillagerInstance::new(
					VillagerArchetype::GoodVillager(GoodVillager::Bishop),
					Some(Expression::Leaf(Testimony::Bishop(BishopClaim::new(
						&[
							VillagerIndex::number(2),
							VillagerIndex::number(4),
							VillagerIndex::number(6),
						],
						true,
						true,
						Some(BishopEvil::Minion),
					)))),
				)),
			)),
			TestAction::TryReveal(RevealResult::new(
				VillagerIndex::number(3),
				Some(VillagerInstance::new(
					VillagerArchetype::GoodVillager(GoodVillager::Confessor),
					Some(Expression::Leaf(Testimony::Confess(ConfessorClaim::Good))),
				)),
			)),
			TestAction::TryReveal(RevealResult::new(
				VillagerIndex::number(4),
				Some(VillagerInstance::new(
					VillagerArchetype::GoodVillager(GoodVillager::Bard),
					Some(Expression::Leaf(Testimony::Bard(Some(
						NonZeroUsize::new(1).unwrap(),
					)))),
				)),
			)),
			TestAction::TryReveal(RevealResult::new(
				VillagerIndex::number(5),
				Some(VillagerInstance::new(
					VillagerArchetype::GoodVillager(GoodVillager::Knitter),
					Some(Expression::Leaf(Testimony::Knitter(EvilPairsClaim::new(2)))),
				)),
			)),
			TestAction::TryReveal(RevealResult::new(
				VillagerIndex::number(6),
				Some(VillagerInstance::new(
					VillagerArchetype::GoodVillager(GoodVillager::Lover),
					Some(Testimony::lover(
						&VillagerIndex::number(6),
						0,
						game_state.total_villagers(),
					)),
				)),
			)),
			TestAction::TryReveal(RevealResult::new(
				VillagerIndex::number(7),
				Some(VillagerInstance::new(
					VillagerArchetype::GoodVillager(GoodVillager::Knight),
					Some(Expression::Leaf(Testimony::Invincible(
						VillagerIndex::number(7),
					))),
				)),
			)),
			TestAction::TryReveal(RevealResult::new(
				VillagerIndex::number(8),
				Some(VillagerInstance::new(
					VillagerArchetype::GoodVillager(GoodVillager::Oracle),
					Some(Testimony::oracle(
						&[VillagerIndex::number(5), VillagerIndex::number(6)],
						VillagerArchetype::Minion(Minion::Puppeteer),
					)),
				)),
			)),
			TestAction::TryExecute(KillAttempt::new(
				VillagerIndex::number(1),
				Some(KillResult::Revealed(
					KillData::new(Some(VillagerArchetype::Minion(Minion::Puppeteer)), false)
						.expect("Bad kill data?"),
				)),
			)),
			TestAction::TryExecute(KillAttempt::new(
				VillagerIndex::number(2),
				Some(KillResult::Revealed(
					KillData::new(Some(VillagerArchetype::Minion(Minion::Puppet)), false)
						.expect("Bad kill data?"),
				)),
			)),
			TestAction::TryExecute(KillAttempt::new(
				VillagerIndex::number(8),
				Some(KillResult::Revealed(
					KillData::new(Some(VillagerArchetype::Demon(Demon::Pooka)), false)
						.expect("Bad kill data?"),
				)),
			)),
		],
		None,
	);
}
//#1: Puppeteer, #2: Puppet, #3: Confessor, #4: Bard, #5: Knitter, #6: Lover, #7: Knight, #8: Oracle (actually a Pooka) - #4 is Drunk - #2 was puppeted by #1 - #7 was corrupted by #8
