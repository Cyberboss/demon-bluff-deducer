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
	collections::{BTreeMap, BTreeSet, HashMap, HashSet, hash_map::Entry},
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

struct PredictionResult {
	board_layouts_by_similar_configs: HashMap<BTreeSet<VillagerIndex>, BTreeSet<BoardLayout>>,
	most_common_indicies: Option<Vec<VillagerIndex>>,
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

	let mut any_revealed = false;
	state.iter_villagers(|_, villager| {
		if let Villager::Hidden(_) = villager {
		} else {
			any_revealed = true;
		}
	});

	if any_revealed {
		// Step one, build possible board layouts as an ExpressionWithTag HashMap<Vec<VillagerArchetype, ExpressionWithTag<Testimony>>>
		match predict_board_configs(log, state, build_board_layouts(state)) {
			Ok(valid_prediction) => {
				if let Some(valid_prediction) = valid_prediction {
					if let Some(most_common_indicies) = valid_prediction.most_common_indicies {
						if most_common_indicies.len() == 1 {
							// TODO: does it make sense to kill ASAP if there's a night affect in play?
							let mut actions = HashSet::with_capacity(1);
							actions.insert(PlayerAction::TryExecute(
								most_common_indicies.into_iter().next().unwrap(),
							));
							return Ok(actions);
						}

						todo!()
					}

					assert_eq!(
						1,
						valid_prediction.board_layouts_by_similar_configs.len(),
						"More than 1 board layout is killable and most_common_indicies wasn't set!"
					);

					return Ok(kill_board_configs(
						valid_prediction
							.board_layouts_by_similar_configs
							.into_iter()
							.next()
							.unwrap()
							.1,
						state,
					));
				}
			}
			Err(prediction_error) => return Err(prediction_error),
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

fn predict_board_configs(
	log: &impl Log,
	game_state: &GameState,
	configs: impl IntoIterator<Item = BoardLayout>,
) -> Result<Option<PredictionResult>, PredictionError> {
	let potential_board_configurations: Vec<BoardLayout> = configs.into_iter().collect();

	if potential_board_configurations.is_empty() {
		return Err(PredictionError::GameUnsolvable);
	}

	// Step two run possibilities, if only one satisfies, execute evils in board layout, if more than one satisfies and at least one evil overlaps on all, execute that one, otherwise, gather more info
	info!(logger: log, "{} potential board configurations with remaining evils", potential_board_configurations.len());
	if potential_board_configurations.len() == 1 {
		let board_config = &potential_board_configurations[0];
		let mut final_configs = HashMap::with_capacity(1);
		final_configs.insert(
			board_config.evil_locations.clone(),
			potential_board_configurations.into_iter().collect(),
		);

		return Ok(Some(PredictionResult {
			board_layouts_by_similar_configs: final_configs,
			most_common_indicies: None,
		}));
	}

	let mut potential_board_expressions = Vec::with_capacity(potential_board_configurations.len());

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
						game_state,
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
			let matching_configs = potential_board_configurations
				.into_iter()
				.filter(|board_config| board_config.evil_locations == matching_layout)
				.collect();

			let mut kills = HashMap::with_capacity(1);
			kills.insert(matching_layout, matching_configs);
			return Ok(Some(PredictionResult {
				board_layouts_by_similar_configs: kills,
				most_common_indicies: None,
			}));
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
			let most_common_index = &most_common_indicies[0];
			let mut matching_configs: HashMap<BTreeSet<VillagerIndex>, BTreeSet<BoardLayout>> =
				HashMap::new();
			for config in potential_board_configurations {
				if config.evil_locations.contains(most_common_index) {
					match matching_configs.entry(config.evil_locations.clone()) {
						Entry::Occupied(mut occupied_entry) => {
							occupied_entry.get_mut().insert(config);
						}
						Entry::Vacant(vacant_entry) => {
							let mut set = BTreeSet::new();
							set.insert(config);
							vacant_entry.insert(set);
						}
					}
				}
			}

			return Ok(Some(PredictionResult {
				board_layouts_by_similar_configs: matching_configs,
				most_common_indicies: Some(most_common_indicies),
			}));
		}

		let mut can_get_more_information = false;

		game_state.iter_villagers(|_, villager| {
			// is there a villager without a testimony or hidden?
			can_get_more_information |= match villager {
				Villager::Active(active_villager) => active_villager.instance().testimony(),
				Villager::Hidden(hidden_villager) => {
					can_get_more_information |= !hidden_villager.cant_reveal();
					return;
				}
				Villager::Confirmed(confirmed_villager) => {
					confirmed_villager.instance().testimony()
				}
			}
			.is_none()
		});

		if can_get_more_information {
			info!(logger: log, "{} different evil layouts exist, need more information!", matching_layouts.len());
			Ok(None)
		} else {
			// best guess
			warn!(
				logger: log,
				"{} different evil layouts exist and no more information can be gathered. Providing the {} most common evil indexes with {} matches each. God help you",
				matching_layouts.len(),most_common_indicies.len(), highest_count
			);
			let mut matching_configs: HashMap<BTreeSet<VillagerIndex>, BTreeSet<BoardLayout>> =
				HashMap::new();
			for config in potential_board_configurations {
				if config
					.evil_locations
					.iter()
					.any(|index| most_common_indicies.contains(index))
				{
					match matching_configs.entry(config.evil_locations.clone()) {
						Entry::Occupied(mut occupied_entry) => {
							occupied_entry.get_mut().insert(config);
						}
						Entry::Vacant(vacant_entry) => {
							let mut set = BTreeSet::new();
							set.insert(config);
							vacant_entry.insert(set);
						}
					}
				}
			}

			return Ok(Some(PredictionResult {
				board_layouts_by_similar_configs: matching_configs,
				most_common_indicies: Some(most_common_indicies),
			}));
		}
	} else {
		info!(logger: log, "No villagers with testimonies, can't predict.");
		Ok(None)
	}
}

fn kill_board_configs(
	board_configs: impl IntoIterator<Item = BoardLayout>,
	state: &GameState,
) -> HashSet<PlayerAction> {
	let mut kills = HashSet::new();

	// TODO: Find most common night effect positions
	let board_config = board_configs.into_iter().next().unwrap();

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
			info!(logger: log, "Validation failed: {}", board_config.description);
			return false;
		}
	}

	info!(logger: log, "Validation passed: {}", board_config.description);
	true
}
