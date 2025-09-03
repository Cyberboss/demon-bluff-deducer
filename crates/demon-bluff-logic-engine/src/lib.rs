#![feature(breakpoint, cold_path, rust_cold_cc, hash_set_entry, gen_blocks)]

mod build_board_layouts;
mod build_expression_for_villager_set;
mod engine;
mod expression_assertion;
mod hypotheses;
mod player_action;
mod prediction_error;

use core::panic;
use std::{
	arch::breakpoint,
	cmp::max,
	collections::{HashMap, HashSet, hash_map::Entry},
	str::FromStr,
};

use build_board_layouts::{BoardLayout, TheoreticalVillager, build_board_layouts};
use build_expression_for_villager_set::{IndexTestimony, build_expression_for_villager_set};
use demon_bluff_gameplay_engine::{
	Expression,
	affect::Affect,
	game_state::{self, GameState},
	testimony::{self, ConfessorClaim, Testimony, index_offset},
	villager::{GoodVillager, Outcast, Villager, VillagerArchetype, VillagerIndex},
};
use engine::evaluate;
use expression_assertion::{collect_satisfying_assignments, evaluate_with_assignment};
use log::{Log, debug, info, warn};

pub use self::{
	engine::{Breakpoint, DebuggerContext, DesireNode, FITNESS_UNIMPLEMENTED, HypothesisNode},
	player_action::PlayerAction,
	prediction_error::PredictionError,
};
use crate::hypotheses::MasterHypothesisBuilder;

pub fn predict_with_debugger<FDebugBreak>(
	log: &impl Log,
	state: &GameState,
	breakpoint_handler: FDebugBreak,
) -> Result<HashSet<PlayerAction>, PredictionError>
where
	FDebugBreak: FnMut(Breakpoint) + Clone,
{
	evaluate(
		state,
		MasterHypothesisBuilder::default(),
		log,
		Some(breakpoint_handler),
	)
}

pub fn predict(
	log: &impl Log,
	state: &GameState,
) -> Result<HashSet<PlayerAction>, PredictionError> {
	/* evaluate(
		state,
		MasterHypothesisBuilder::default(),
		log,
		None::<fn(Breakpoint)>,
	)*/

	// Step one, build possible board layouts as an ExpressionWithTag HashMap<Vec<VillagerArchetype, ExpressionWithTag<Testimony>>>
	let potential_board_configurations: Vec<BoardLayout> =
		build_board_layouts(state).into_iter().collect();

	// Step two run possibilities, if only one satisfies, execute evils in board layout, if more than one satisfies and at least one evil overlaps on all, execute that one, otherwise, gather more info
	info!(logger: log, "{} potential board configurations with remaining evils", potential_board_configurations.len());
	if potential_board_configurations.len() > 0 {
		if potential_board_configurations.len() == 1 {
			let board_config: BoardLayout =
				potential_board_configurations.into_iter().next().unwrap();
			return Ok(kill_board_config(board_config, state));
		}

		let mut potential_board_expressions =
			Vec::with_capacity(potential_board_configurations.len());

		let mut master_expression = None;
		for config_expression in potential_board_configurations
			.iter()
			.filter_map(|board_config| {
				build_expression_for_villager_set(
					board_config
						.villagers
						.iter()
						.map(|theoretical_villager| &theoretical_villager.inner),
				)
			}) {
			master_expression = Some(match master_expression {
				Some(previous_expression) => Expression::Or(
					Box::new(previous_expression),
					Box::new(config_expression.clone()),
				),
				None => config_expression.clone(),
			});
			potential_board_expressions.push(config_expression);
		}

		if let Some(master_expression) = master_expression {
			let potential_assignments = collect_satisfying_assignments(&master_expression);
			if potential_assignments.is_empty() {
				return Err(PredictionError::GameUnsolvable);
			}

			let mut matching_layouts = HashSet::new();
			let mut matching_configs = 0;
			for (index, board_expression) in potential_board_expressions.into_iter().enumerate() {
				for assignment in &potential_assignments {
					if evaluate_with_assignment(&board_expression, &assignment)
						&& validate_assignment(
							log,
							assignment,
							&potential_board_configurations[index],
							state,
						) {
						matching_configs += 1;
						matching_layouts
							.insert(potential_board_configurations[index].evil_locations.clone());
					}
				}
			}

			let mut layout_number = 0;
			info!(logger: log, "Filtered to {} evil layouts amongst {} configurations", matching_layouts.len(), matching_configs);
			for layout in &matching_layouts {
				layout_number += 1;
				info!(logger: log, "Potential Layout {}", layout_number);
				for index in layout {
					info!(logger: log, "- {}", index);
				}
			}

			if matching_layouts.len() == 1 {
				let matching_layout = matching_layouts.into_iter().next().unwrap();
				let mut kills = HashSet::with_capacity(matching_layout.len());
				kills.extend(
					matching_layout
						.into_iter()
						.map(|index| PlayerAction::TryExecute(index)),
				);
				return Ok(kills);
			}

			let mut overlapping_evil_indexes = HashMap::new();
			let mut highest_count = 0;
			for layout in &matching_layouts {
				for index in layout {
					let entry = overlapping_evil_indexes.entry(index.clone());
					match entry {
						Entry::Occupied(occupied_entry) => {
							let new_result = 1 + occupied_entry.get();
							highest_count = max(highest_count, new_result);
							overlapping_evil_indexes.insert(index.clone(), new_result);
						}
						Entry::Vacant(vacant_entry) => {
							highest_count = max(highest_count, 1);
							vacant_entry.insert(1);
						}
					}
				}
			}

			let mut most_common_indicies = Vec::new();
			for (index, count) in overlapping_evil_indexes {
				if count == highest_count {
					most_common_indicies.push(index);
				}
			}

			if most_common_indicies.len() == 1 {
				let mut kills = HashSet::new();
				kills.insert(PlayerAction::TryExecute(most_common_indicies[0].clone()));
				return Ok(kills);
			}

			let mut can_get_more_information = false;

			state.iter_villagers(|_, villager| {
				// is there a villager without a testimony or hidden?
				can_get_more_information = match villager {
					Villager::Active(active_villager) => active_villager.instance().testimony(),
					Villager::Hidden(_) => {
						can_get_more_information = true;
						return;
					}
					Villager::Confirmed(confirmed_villager) => {
						confirmed_villager.instance().testimony()
					}
				}
				.is_none()
			});

			if can_get_more_information {
				info!(
						logger: log, "{} different evil layouts exist, need more information!", matching_layouts.len());
			} else {
				// best guess
				// TODO: Prioritize night effects
				warn!(
					logger: log,
					"{} different evil layouts exist and no more information can be gathered. Providing the {} most common evil indexes with {} matches each. God help you",
					matching_layouts.len(),most_common_indicies.len(), highest_count
				);
				return Ok(most_common_indicies
					.into_iter()
					.map(|index| PlayerAction::TryExecute(index))
					.collect());
			}
		}
	}

	// Step three, need more info. Figure out how to best use reveals/abilities to gain info
	// For now just reveal the first hidden index and we'll make it better later
	let mut hidden_index = None;
	state.iter_villagers(|index, villager| {
		if hidden_index.is_none()
			&& let Villager::Hidden(_) = villager
		{
			hidden_index = Some(index);
		}
	});

	let mut actions = HashSet::new();

	actions.insert(PlayerAction::TryReveal(hidden_index.unwrap_or_else(|| {
		todo!("Not enough info to execute and no villagers left to reveal!")
	})));

	Ok(actions)
}

fn kill_board_config(board_config: BoardLayout, state: &GameState) -> HashSet<PlayerAction> {
	let mut kills = HashSet::new();

	// minor thing, important to kill evils with night effects first
	for evil_index in &board_config.evil_locations {
		if let Some(Affect::Night(_)) = board_config.villagers[evil_index.0]
			.inner
			.true_identity()
			.affect(state.total_villagers(), Some(evil_index.clone()))
		{
			kills.insert(PlayerAction::TryExecute(evil_index.clone()));
			return kills;
		}
	}

	for evil_index in board_config.evil_locations {
		kills.insert(PlayerAction::TryExecute(evil_index));
	}

	kills
}

fn validate_assignment(
	log: &impl Log,
	assignment: &HashMap<IndexTestimony, bool>,
	board_config: &BoardLayout,
	game_state: &GameState,
) -> bool {
	let theoreticals = &board_config.villagers;

	for (index_testimony, truthful) in assignment {
		let testimony_valid = match &index_testimony.testimony {
			Testimony::Good(villager_index) => !theoreticals[villager_index.0]
				.inner
				.true_identity()
				.is_evil(),
			Testimony::Evil(villager_index) => theoreticals[villager_index.0]
				.inner
				.true_identity()
				.is_evil(),
			Testimony::Corrupt(villager_index) => theoreticals[villager_index.0].inner.corrupted(),
			Testimony::NotCorrupt(villager_index) => {
				!theoreticals[villager_index.0].inner.corrupted()
			}
			Testimony::Lying(villager_index) => {
				let confirmed = &theoreticals[villager_index.0].inner;
				confirmed.corrupted()
					|| (!confirmed.instance().archetype().cannot_lie()
						&& confirmed.true_identity().lies())
			}
			Testimony::Cured(villager_index) => {
				let theoretical = &theoreticals[villager_index.0];
				theoretical.was_corrupt && !theoretical.inner.corrupted()
			}
			Testimony::Baker(baker_claim) => {
				let theoretical = &theoreticals[index_testimony.index.0];
				theoretical.baked_from == *baker_claim.was()
			}
			Testimony::Role(role_claim) => {
				theoreticals[role_claim.index().0].inner.true_identity() == role_claim.role()
			}
			Testimony::Invincible(villager_index) => {
				*theoreticals[villager_index.0].inner.true_identity()
					== VillagerArchetype::GoodVillager(GoodVillager::Knight)
			}
			Testimony::Affected(affected_claim) => {
				theoreticals[affected_claim.index().0].affection.as_ref()
					== Some(affected_claim.affect_type())
			}
			Testimony::FakeEvil(villager_index) => {
				*theoreticals[villager_index.0].inner.true_identity()
					== VillagerArchetype::Outcast(Outcast::Wretch)
			}
			Testimony::SelfDestruct(villager_index) => {
				*theoreticals[villager_index.0].inner.true_identity()
					== VillagerArchetype::Outcast(Outcast::Bombardier)
			}
			Testimony::Slayed(slay_result) => {
				if slay_result.slayed() {
					true
				} else {
					let confirmed_target = &theoreticals[slay_result.index().0].inner;
					let confirmed_me = &theoreticals[index_testimony.index.0].inner;
					!confirmed_target.true_identity().is_evil() || confirmed_me.corrupted()
				}
			}
			Testimony::Confess(confession) => {
				let confirmed = &theoreticals[index_testimony.index.0].inner;
				match confession {
					ConfessorClaim::Good => {
						!confirmed.corrupted() && !confirmed.true_identity().is_evil()
					}
					ConfessorClaim::Dizzy => {
						confirmed.corrupted() || confirmed.true_identity().is_evil()
					}
				}
			}
			Testimony::Scout(scout_claim) => {
				let likely_talking_about = theoreticals
					.iter()
					.filter(|theoretical| {
						theoretical.inner.true_identity() == scout_claim.evil_role()
					})
					.enumerate()
					.next();

				match likely_talking_about {
					Some((target_index, _)) => {
						let clockwise_read = index_offset(
							&VillagerIndex(target_index),
							game_state.total_villagers(),
							scout_claim.distance(),
							true,
						);
						let counterclockwise_read = index_offset(
							&VillagerIndex(target_index),
							game_state.total_villagers(),
							scout_claim.distance(),
							true,
						);

						// TODO: Need to check there are no closer eviles in either direction
						theoreticals[counterclockwise_read.0]
							.inner
							.true_identity()
							.is_evil() || theoreticals[clockwise_read.0]
							.inner
							.true_identity()
							.is_evil()
					}
					None => false,
				}
			}
		};

		if testimony_valid != *truthful {
			debug!(logger: log, "Validation failed: {}", board_config.description);
			return false;
		}
	}

	debug!(logger: log, "Validation passed: {}", board_config.description);
	true
}
