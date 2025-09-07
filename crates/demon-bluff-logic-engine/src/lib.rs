#![feature(
	breakpoint,
	cold_path,
	rust_cold_cc,
	hash_set_entry,
	gen_blocks,
	maybe_uninit_slice,
	maybe_uninit_fill
)]

mod build_board_layouts;
mod build_expression_for_villager_set;
mod expression_assertion;
mod optimized_expression;
mod player_action;
mod prediction_error;
mod reveal_strategy;
mod with_theoretical_testimony;

use core::panic;
use std::{
	arch::breakpoint,
	cmp::max,
	collections::{BTreeSet, HashMap, HashSet, hash_map::Entry},
	sync::atomic::{AtomicI32, Ordering},
	usize,
};

use build_board_layouts::{BoardLayout, TheoreticalVillager, build_board_layouts};
use build_expression_for_villager_set::{IndexTestimony, build_expression_for_villager_set};
use demon_bluff_gameplay_engine::{
	Expression,
	affect::Affect,
	game_state::GameState,
	testimony::{ConfessorClaim, Direction, Testimony, index_offset},
	villager::{Demon, GoodVillager, Minion, Outcast, Villager, VillagerArchetype, VillagerIndex},
};
use expression_assertion::{collect_satisfying_assignments, evaluate_with_assignment};
use itertools::Itertools;
use log::{Log, debug, info, warn};
use optimized_expression::OptimizedExpression;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use with_theoretical_testimony::with_theoretical_testimony;

pub use self::{
	player_action::{AbilityAttempt, PlayerAction},
	prediction_error::PredictionError,
	reveal_strategy::RevealStrategy,
};

struct PredictionResult {
	all_matching_layouts: HashMap<BoardLayout, Vec<HashMap<IndexTestimony, bool>>>,
	board_layouts_by_similar_configs: HashMap<BTreeSet<VillagerIndex>, BTreeSet<BoardLayout>>,
	most_common_indicies: Option<(Vec<VillagerIndex>, Option<usize>)>,
}

pub fn predict(
	log: &impl Log,
	state: &GameState,
	reveal_strategy: RevealStrategy,
) -> Result<HashSet<PlayerAction>, PredictionError> {
	let mut any_revealed = false;
	state.iter_villagers(|_, villager| {
		if let Villager::Hidden(_) = villager {
			true
		} else {
			any_revealed = true;
			false
		}
	});

	let mut need_more_info_result = None;
	if any_revealed {
		let initial_layouts = build_board_layouts(state);

		match predict_core(
			log,
			state,
			initial_layouts
				.into_iter()
				.map(|board_layout| (board_layout, None)),
			false,
		) {
			PredictionResult2::KillResult(hash_set) => {
				return hash_set;
			}
			PredictionResult2::NeedMoreInfoResult(hash_set) => {
				need_more_info_result = Some(hash_set)
			}
			PredictionResult2::ConfigCountsAfterAbility(_) => panic!("Incorrect return type!"),
		}
	}

	// Step three, need more info. Figure out how to best use reveals/abilities to gain info
	// For now just reveal the first hidden index and we'll make it better later
	let mut revealable_index = None;
	state.iter_villagers(|index, villager| {
		if revealable_index.is_none()
			&& let Villager::Hidden(hidden_villager) = villager
			&& !hidden_villager.cant_reveal()
		{
			revealable_index = Some(index);
			false
		} else {
			true
		}
	});

	match revealable_index {
		Some(_) => Ok(reveal_strategy.get_reveal(log, state)),
		None => {
			let mut remaining_unused_abilities = 0;
			state.iter_villagers(|_, villager| {
				match villager {
					Villager::Active(active_villager) => {
						if active_villager.instance().testimony().is_none() {
							remaining_unused_abilities += 1;
						}
					}
					Villager::Hidden(_) => {}
					Villager::Confirmed(confirmed_villager) => {
						if confirmed_villager.instance().testimony().is_none() {
							remaining_unused_abilities += 1;
						}
					}
				}
				true
			});

			info!(logger: log, "We must try to use an ability. Predicting outcomes of remaining {} unused abilities", remaining_unused_abilities);

			let initial_layouts = need_more_info_result.expect("Udhfhfhfh");

			let mut layouts = with_theoretical_testimony(log, state, initial_layouts);

			let mut attempt_order = Vec::with_capacity(layouts.len());
			attempt_order.extend(
				layouts
					.iter()
					.map(|(ability_attempt, _)| ability_attempt)
					.cloned(),
			);
			attempt_order.sort();

			// evaluating every ability combination is too damn expensive (and impossible to empiracally verify in the game)
			// order here wants to be deterministic for testing purposes, so collect and sort the keys

			// for each theoretical testimony find the group of
			let mut least_options: Option<(HashSet<AbilityAttempt>, usize)> = None;
			for ability_attempt in attempt_order {
				let (_, predicted_layouts) =
					layouts.remove_entry(&ability_attempt).expect("Impossble");
				if let PredictionResult2::ConfigCountsAfterAbility(result) = predict_core(
					log,
					state,
					predicted_layouts
						.into_iter()
						.map(|(board_config, potential_assignments)| {
							(board_config, Some(potential_assignments))
						}),
					true,
				) {
					let (ability_uses, potential_evil_location_configurations) = match least_options
					{
						Some((mut old_least_options, count)) => {
							if count > result {
								let mut new_least_options = HashSet::new();
								new_least_options.insert(ability_attempt);
								(new_least_options, result)
							} else {
								if count == result {
									old_least_options.insert(ability_attempt);
								}

								(old_least_options, count)
							}
						}
						None => {
							let mut new_least_options = HashSet::new();
							new_least_options.insert(ability_attempt);
							(new_least_options, result)
						}
					};

					// optimization, take the first result that gives us all remaining evils
					let this_one_works = potential_evil_location_configurations == 1;

					least_options = Some((ability_uses, potential_evil_location_configurations));

					if this_one_works {
						break;
					}
				} else {
					panic!("Prediction was not allowed to return non and it did it anyway");
				}
			}

			Ok(least_options
				.expect("No value ability usages found??")
				.0
				.into_iter()
				.map(|ability_attempt| PlayerAction::Ability(ability_attempt))
				.collect())
		}
	}
}

enum PredictionResult2 {
	KillResult(Result<HashSet<PlayerAction>, PredictionError>),
	ConfigCountsAfterAbility(usize),
	NeedMoreInfoResult(HashMap<BoardLayout, Vec<HashMap<IndexTestimony, bool>>>),
}

enum PredictionResult3 {
	PredictionResult(PredictionResult),
	NeedMoreInfoResult(HashMap<BoardLayout, Vec<HashMap<IndexTestimony, bool>>>),
}

fn predict_core(
	log: &impl Log,
	state: &GameState,
	layouts: impl Iterator<Item = (BoardLayout, Option<Vec<HashMap<IndexTestimony, bool>>>)>,
	count_configs: bool,
) -> PredictionResult2 {
	// Step one, build possible board layouts as an ExpressionWithTag HashMap<Vec<VillagerArchetype, ExpressionWithTag<Testimony>>>
	let prediction_result = predict_board_configs(log, state, layouts, !count_configs);
	match prediction_result {
		Ok(valid_prediction) => {
			match valid_prediction {
				PredictionResult3::PredictionResult(valid_prediction) => {
					if count_configs {
						// we actually want to eliminate board layouts that have narrowed things down to the remaining evils

						let result = PredictionResult2::ConfigCountsAfterAbility(
							valid_prediction.board_layouts_by_similar_configs.len(),
						);

						return result;
					}

					if let Some((most_common_indicies, appear_in_layouts)) =
						valid_prediction.most_common_indicies
					{
						let mut actions: HashSet<PlayerAction> =
							HashSet::with_capacity(most_common_indicies.len());
						// select the most common indicies
						for index in most_common_indicies {
							// TODO: does it make sense to kill ASAP if there's a night affect in play?
							actions.insert(PlayerAction::TryExecute(index));
						}

						assert_ne!(0, actions.len());

						return PredictionResult2::KillResult(Ok(actions));
					}

					assert_eq!(
						1,
						valid_prediction.board_layouts_by_similar_configs.len(),
						"More than 1 board layout is killable and most_common_indicies wasn't set!"
					);

					PredictionResult2::KillResult(Ok(kill_board_configs(
						valid_prediction
							.all_matching_layouts
							.into_iter()
							.map(|(layout, _)| layout),
						state,
					)))
				}
				PredictionResult3::NeedMoreInfoResult(hash_set) => {
					PredictionResult2::NeedMoreInfoResult(hash_set)
				}
			}
		}
		Err(prediction_error) => PredictionResult2::KillResult(Err(prediction_error)),
	}
}

fn predict_board_configs(
	log: &impl Log,
	game_state: &GameState,
	configs: impl Iterator<Item = (BoardLayout, Option<Vec<HashMap<IndexTestimony, bool>>>)>,
	allow_retry: bool,
) -> Result<PredictionResult3, PredictionError> {
	let potential_board_configurations: Vec<(
		BoardLayout,
		Option<Vec<HashMap<IndexTestimony, bool>>>,
	)> = configs.into_iter().collect();

	if potential_board_configurations.is_empty() {
		return Err(PredictionError::GameUnsolvable);
	}

	// Step two run possibilities, if only one satisfies, execute evils in board layout, if more than one satisfies and at least one evil overlaps on all, execute that one, otherwise, gather more info
	info!(logger: log, "{} potential board configurations with remaining evils", potential_board_configurations.len());
	if potential_board_configurations.len() == 1 {
		let (board_config, _) = &potential_board_configurations[0];
		let mut final_configs = HashMap::with_capacity(1);
		final_configs.insert(
			board_config.evil_locations.clone(),
			potential_board_configurations
				.iter()
				.map(|(config, _)| config)
				.cloned()
				.collect(),
		);

		return Ok(PredictionResult3::PredictionResult(PredictionResult {
			all_matching_layouts: potential_board_configurations
				.into_iter()
				.map(|(layout, _)| (layout, Vec::new()))
				.collect(),
			board_layouts_by_similar_configs: final_configs,
			most_common_indicies: None,
		}));
	}

	let mut potential_board_expressions = Vec::with_capacity(potential_board_configurations.len());

	let mut master_expression = None;
	for config_expression in
		potential_board_configurations
			.iter()
			.filter_map(|(board_config, _)| {
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
		let optimized_master_expression = OptimizedExpression::new(&master_expression);

		let optimized_expressions: Vec<OptimizedExpression<IndexTestimony>> =
			potential_board_expressions
				.iter()
				.map(|board_expression| OptimizedExpression::new(&board_expression))
				.collect();

		let mut potential_assignment_mappings = if !allow_retry {
			Some(Vec::with_capacity(potential_board_configurations.len()))
		} else {
			None
		};

		let potential_assignments = if allow_retry {
			collect_satisfying_assignments(&optimized_master_expression)
		} else {
			// all board configurations should have potential assignments alongside them and they should NOT overlap
			// convert them to optimized form
			potential_board_configurations
				.iter()
				.enumerate()
				.flat_map(|(index, (_, potential_assignments))| {
					potential_assignments
						.as_ref()
						.expect("We should have potential assignments predicted!")
						.iter()
						.map(move |potential_assignment| (index, potential_assignment))
				})
				.map(|(index, potential_assignment)| {
					let mut assignment_vec =
						Vec::with_capacity(optimized_master_expression.variables().len());
					let mut mapping_vec: Vec<bool> = Vec::with_capacity(assignment_vec.len());
					let mut num_used_from_potential_assignment = 0;

					for variable in optimized_master_expression.variables().iter() {
						assignment_vec.push(match potential_assignment.get(&variable) {
							Some(testimony_trutfulness) => {
								num_used_from_potential_assignment += 1;
								mapping_vec.push(true);
								*testimony_trutfulness
							}
							None => {
								mapping_vec.push(false);
								false // shouldn't matter the value here, but definitely reconsider if predictions get weird
							}
						});
					}

					if optimized_expressions[index].variables().len()
						!= num_used_from_potential_assignment
					{
						panic!("Bad juju");
					}

					potential_assignment_mappings
						.as_mut()
						.unwrap()
						.push((mapping_vec, num_used_from_potential_assignment));

					assignment_vec
				})
				.collect()
		};
		if potential_assignments.is_empty() {
			return Err(PredictionError::GameUnsolvable);
		}

		info!(
			logger: log,
			"{} potential assignments to evaluate",
			potential_assignments.len()
		);
		let mut all_matching_layouts: HashMap<BoardLayout, Vec<HashMap<IndexTestimony, bool>>> =
			HashMap::new();
		let mut matching_layouts = HashSet::new();

		let things_to_check: Vec<(usize, &OptimizedExpression<IndexTestimony>, &Vec<bool>)> =
			optimized_expressions
				.iter()
				.enumerate()
				.flat_map(|(index, board_expression)| {
					let vec: Vec<(usize, &OptimizedExpression<IndexTestimony>, &Vec<bool>)> =
						potential_assignments
							.iter()
							.map(|assignment| (index, board_expression, assignment))
							.collect();
					vec
				})
				.collect();

		let matching_configs = AtomicI32::new(0);
		let wretch_in_play = game_state.role_in_play(VillagerArchetype::Outcast(Outcast::Wretch));
		let drunk_in_play = game_state.role_in_play(VillagerArchetype::Outcast(Outcast::Drunk));
		let knight_in_play =
			game_state.role_in_play(VillagerArchetype::GoodVillager(GoodVillager::Knight));
		let bombardier_in_play =
			game_state.role_in_play(VillagerArchetype::Outcast(Outcast::Bombardier));
		let iteration_result: Vec<(usize, HashMap<IndexTestimony, bool>)> = things_to_check
			.into_par_iter()
			.filter_map(|(index, board_expression, assignment)| {
				let mapped_assignment;
				let assignment = if allow_retry {
					assignment
				} else {
					let (mapping, original_slots_used) =
						&potential_assignment_mappings.as_ref().unwrap()[index];

					if *original_slots_used != board_expression.variables().len() {
						panic!("Bad juju");
					}

					let mut mapped_assignment_builder = Vec::with_capacity(*original_slots_used);

					for i in 0..mapping.len() {
						if mapping[i] {
							mapped_assignment_builder.push(assignment[i]);
						}
					}

					mapped_assignment = Some(mapped_assignment_builder);
					mapped_assignment.as_ref().unwrap()
				};

				if board_expression.satisfies(|variable_index| assignment[variable_index])
					&& validate_assignment(
						log,
						assignment,
						board_expression.variables(),
						&potential_board_configurations[index].0,
						game_state,
						wretch_in_play,
						drunk_in_play,
						knight_in_play,
						bombardier_in_play,
					) {
					matching_configs.fetch_add(1, Ordering::Relaxed);

					let mut satisfying_assignment =
						HashMap::with_capacity(board_expression.variables().len());
					for (index, variable) in board_expression.variables().iter().enumerate() {
						satisfying_assignment.insert(variable.clone(), assignment[index]);
					}

					Some((index, satisfying_assignment))
				} else {
					None
				}
			})
			.collect();

		for (matching_board_config_index, satisfying_assignment) in iteration_result {
			let (matching_board_config, _) =
				&potential_board_configurations[matching_board_config_index];
			if all_matching_layouts.contains_key(matching_board_config) {
				let satisfying_assignments = all_matching_layouts
					.get_mut(matching_board_config)
					.expect("Should have been initilized in the below line");
				satisfying_assignments.push(satisfying_assignment);
			} else {
				all_matching_layouts
					.insert(matching_board_config.clone(), vec![satisfying_assignment]);
				matching_layouts.insert(matching_board_config.evil_locations.clone());
			}
		}

		let mut layout_number = 0;
		let matching_configs = matching_configs.fetch_add(0, Ordering::Acquire);
		info!(logger: log, "Filtered to {} evil layouts amongst {} configurations", matching_layouts.len(), matching_configs);
		for layout in &matching_layouts {
			layout_number += 1;
			info!(logger: log, "Potential Layout {}", layout_number);
			for index in layout {
				info!(logger: log, "- {}", index);
			}
		}

		for (index, (matching_layout, _)) in all_matching_layouts.iter().enumerate() {
			info!(logger: log, "Layout #{}: {}", index + 1, matching_layout.description);
		}

		if matching_layouts.len() == 1 {
			let matching_layout = matching_layouts.into_iter().next().unwrap();
			let matching_configs = potential_board_configurations
				.into_iter()
				.filter_map(|(board_config, _)| {
					if board_config.evil_locations == matching_layout {
						Some(board_config)
					} else {
						None
					}
				})
				.collect();

			let mut kills = HashMap::with_capacity(1);
			kills.insert(matching_layout, matching_configs);
			return Ok(PredictionResult3::PredictionResult(PredictionResult {
				all_matching_layouts,
				board_layouts_by_similar_configs: kills,
				most_common_indicies: None,
			}));
		}

		let mut evil_index_occurrences_in_matching_layouts = HashMap::new();
		let mut highest_count = 0;
		for layout in &matching_layouts {
			for index in layout {
				let entry = evil_index_occurrences_in_matching_layouts.entry(index.clone());
				match entry {
					Entry::Occupied(occupied_entry) => {
						let new_result = 1 + occupied_entry.get();
						highest_count = max(highest_count, new_result);
						evil_index_occurrences_in_matching_layouts
							.insert(index.clone(), new_result);
					}
					Entry::Vacant(vacant_entry) => {
						highest_count = max(highest_count, 1);
						vacant_entry.insert(1);
					}
				}
			}
		}

		let mut most_common_evil_index_occurrences = Vec::new();
		for (index, count) in &evil_index_occurrences_in_matching_layouts {
			if *count == highest_count {
				most_common_evil_index_occurrences.push(index.clone());
			}
		}

		let mut can_get_more_information = false;

		if allow_retry {
			game_state.iter_villagers(|_, villager| {
				// is there a villager without a testimony or hidden?
				can_get_more_information |= match villager {
					Villager::Active(active_villager) => active_villager.instance().testimony(),
					Villager::Hidden(hidden_villager) => {
						if !hidden_villager.cant_reveal() {
							can_get_more_information = true;
						}

						return !can_get_more_information;
					}
					Villager::Confirmed(confirmed_villager) => {
						confirmed_villager.instance().testimony()
					}
				}
				.is_none();
				!can_get_more_information
			});
		}

		if !can_get_more_information
			&& most_common_evil_index_occurrences.len() == 1
			&& *(evil_index_occurrences_in_matching_layouts
				.get(&most_common_evil_index_occurrences[0])
				.unwrap()) == all_matching_layouts.len()
		{
			let most_common_index = &most_common_evil_index_occurrences[0];
			warn!(logger: log, "We found the most common evil index ({} @ {} occurrences across {} layouts) but we are uncertain!", most_common_index, highest_count, matching_layouts.len());
			let mut matching_configs: HashMap<BTreeSet<VillagerIndex>, BTreeSet<BoardLayout>> =
				HashMap::new();
			for (config, _) in all_matching_layouts.iter() {
				match matching_configs.entry(config.evil_locations.clone()) {
					Entry::Occupied(mut occupied_entry) => {
						occupied_entry.get_mut().insert(config.clone());
					}
					Entry::Vacant(vacant_entry) => {
						let mut set = BTreeSet::new();
						set.insert(config.clone());
						vacant_entry.insert(set);
					}
				}
			}

			return Ok(PredictionResult3::PredictionResult(PredictionResult {
				all_matching_layouts,
				board_layouts_by_similar_configs: matching_configs,
				most_common_indicies: Some((
					most_common_evil_index_occurrences,
					Some(highest_count),
				)),
			}));
		}

		if can_get_more_information {
			info!(logger: log, "{} different evil layouts exist, need more information!", matching_layouts.len());
			Ok(PredictionResult3::NeedMoreInfoResult(all_matching_layouts))
		} else {
			// best guess
			if allow_retry {
				warn!(
					logger: log,
					"{} different evil layouts exist and no more information can be gathered. Providing the {} most common evil indexes with {} matches each. God help you",
					matching_layouts.len(),most_common_evil_index_occurrences.len(), highest_count
				);
			}

			let mut board_layouts_by_similar_configs: HashMap<
				BTreeSet<VillagerIndex>,
				BTreeSet<BoardLayout>,
			> = HashMap::new();
			for (config, _) in &all_matching_layouts {
				match board_layouts_by_similar_configs.entry(config.evil_locations.clone()) {
					Entry::Occupied(mut occupied_entry) => {
						occupied_entry.get_mut().insert(config.clone());
					}
					Entry::Vacant(vacant_entry) => {
						let mut set = BTreeSet::new();
						set.insert(config.clone());
						vacant_entry.insert(set);
					}
				}
			}

			if allow_retry {
				// always perfer to kill hidden villagers if necessary to get more info
				let mut has_hiddens = false;
				let mut has_knights = false;
				for index in &most_common_evil_index_occurrences {
					let villager = game_state.villager(index);
					if let Villager::Hidden(_) = villager {
						has_hiddens = true;
						break;
					} else if let Villager::Active(active_villager) = villager {
						if *active_villager.instance().archetype()
							== VillagerArchetype::GoodVillager(GoodVillager::Knight)
						{
							has_knights = true;
						}
					}
				}

				if has_hiddens {
					info!(logger: log, "One or more of the choices is a hidden villager. We will aim to kill these for more information.");
					most_common_evil_index_occurrences.retain(|index| {
						if let Villager::Hidden(_) = game_state.villager(index) {
							true
						} else {
							false
						}
					});
				} else if has_knights {
					info!(logger: log, "One or more of the choices is a knight. We will aim to kill these first to attempt to retain health.");
					most_common_evil_index_occurrences.retain(|index| {
						if let Villager::Active(active_villager) = game_state.villager(index) {
							*active_villager.instance().archetype()
								== VillagerArchetype::GoodVillager(GoodVillager::Knight)
						} else {
							false
						}
					});
				}
			}

			return Ok(PredictionResult3::PredictionResult(PredictionResult {
				all_matching_layouts,
				board_layouts_by_similar_configs,
				most_common_indicies: Some((most_common_evil_index_occurrences, None)),
			}));
		}
	} else {
		info!(logger: log, "No villagers with testimonies, can't predict.");
		Ok(PredictionResult3::NeedMoreInfoResult(
			potential_board_configurations
				.into_iter()
				.map(|(board_layout, _)| (board_layout, Vec::new()))
				.collect(),
		))
	}
}

fn kill_board_configs(
	board_configs: impl Iterator<Item = BoardLayout>,
	state: &GameState,
) -> HashSet<PlayerAction> {
	let mut kills = HashSet::new();

	// TODO: Find most common night effect positions
	for board_config in board_configs {
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
	}

	kills
}

fn validate_assignment(
	log: &impl Log,
	assignment: &Vec<bool>,
	variables: &[IndexTestimony],
	board_config: &BoardLayout,
	game_state: &GameState,
	wretch_in_play: bool,
	drunk_in_play: bool,
	knight_in_play: bool,
	bombardier_in_play: bool,
) -> bool {
	if variables.len() != assignment.len() {
		panic!("This again");
	}

	let theoreticals = &board_config.villagers;
	for (variable_index, truthful) in assignment.iter().enumerate() {
		let index_testimony = &variables[variable_index];
		let testifier = &theoreticals[index_testimony.index.0];
		if !assignment_applies(testifier, &index_testimony.testimony) {
			continue;
		}

		let if_unknown_good_use_truthful = |theoretical: &TheoreticalVillager, input, condition| {
			// if it's an unrevealed good villager, consider this true until proven guilty
			if theoretical.unknown_unrevealed_good() && condition {
				*truthful
			} else {
				input
			}
		};

		let testimony_valid = match &index_testimony.testimony {
			Testimony::Good(villager_index) => {
				let theoretical = &theoreticals[villager_index.0];
				if_unknown_good_use_truthful(
					theoretical,
					!theoretical.inner.true_identity().is_evil(),
					wretch_in_play,
				)
			}
			Testimony::Evil(villager_index) => {
				let theoretical = &theoreticals[villager_index.0];
				if_unknown_good_use_truthful(
					theoretical,
					theoretical.inner.true_identity().appears_evil(),
					wretch_in_play,
				)
			}
			Testimony::Corrupt(villager_index) => theoreticals[villager_index.0].inner.corrupted(),
			Testimony::Lying(villager_index) => {
				let theoretical = &theoreticals[villager_index.0];
				if_unknown_good_use_truthful(
					theoretical,
					theoretical.inner.will_lie(),
					drunk_in_play,
				)
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
				let theoretical = &theoreticals[role_claim.index().0];

				if_unknown_good_use_truthful(
					theoretical,
					theoretical.inner.true_identity() == role_claim.role(),
					!role_claim.role().is_evil(),
				)
			}
			Testimony::Invincible(villager_index) => {
				let theoretical = &theoreticals[villager_index.0];

				if_unknown_good_use_truthful(
					theoretical,
					*theoretical.inner.true_identity()
						== VillagerArchetype::GoodVillager(GoodVillager::Knight),
					knight_in_play,
				)
			}
			Testimony::Affected(affected_claim) => {
				theoreticals[affected_claim.index().0].affection.as_ref()
					== Some(affected_claim.affect_type())
			}
			Testimony::FakeEvil(villager_index) => {
				let theoretical = &theoreticals[villager_index.0];

				if_unknown_good_use_truthful(
					theoretical,
					*theoretical.inner.true_identity()
						== VillagerArchetype::Outcast(Outcast::Wretch),
					wretch_in_play,
				)
			}
			Testimony::SelfDestruct(villager_index) => {
				let theoretical = &theoreticals[villager_index.0];

				if_unknown_good_use_truthful(
					theoretical,
					*theoretical.inner.true_identity()
						== VillagerArchetype::Outcast(Outcast::Bombardier),
					bombardier_in_play,
				)
			}
			Testimony::Slayed(slay_result) => {
				if slay_result.slayed() {
					true
				} else {
					let confirmed_target = &theoreticals[slay_result.index().0].inner;
					let theoretical = &theoreticals[index_testimony.index.0];
					if_unknown_good_use_truthful(
						theoretical,
						!confirmed_target.true_identity().is_evil(),
						true,
					) || theoretical.inner.corrupted()
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
				let mut iterator = theoreticals.iter().enumerate().filter(|(_, theoretical)| {
					theoretical.inner.true_identity() == scout_claim.evil_role()
				});
				let likely_talking_about = iterator.next();

				match likely_talking_about {
					Some((target_index, _)) => {
						if iterator.next().is_some() {
							todo!("Handle multiple matches in scout claim!");
						}

						let target_index = VillagerIndex(target_index);

						let matches;
						let mut i = 0;
						loop {
							i += 1;
							let clockwise_read =
								index_offset(&target_index, game_state.total_villagers(), i, true);
							let counterclockwise_read =
								index_offset(&target_index, game_state.total_villagers(), i, false);

							let clockwise_theoretical = &theoreticals[clockwise_read.0];
							let counterclockwise_theoretical =
								&theoreticals[counterclockwise_read.0];

							let unknown_good_villager_appears_evil =
								(i == scout_claim.distance()) == *truthful;
							let clockwise_appears_evil =
								if clockwise_theoretical.unknown_unrevealed_good() {
									unknown_good_villager_appears_evil
								} else {
									clockwise_theoretical.inner.true_identity().appears_evil()
								};
							let counterclockwise_appears_evil =
								if counterclockwise_theoretical.unknown_unrevealed_good() {
									unknown_good_villager_appears_evil
								} else {
									counterclockwise_theoretical
										.inner
										.true_identity()
										.appears_evil()
								};
							if clockwise_appears_evil || counterclockwise_appears_evil {
								matches = i == scout_claim.distance();
								break;
							}
						}

						matches
					}
					None => false,
				}
			}
			Testimony::Enlightened(direction) => {
				let expected_direction;
				let mut i = 0;
				loop {
					i += 1;
					let clockwise_read = index_offset(
						&index_testimony.index,
						game_state.total_villagers(),
						i,
						true,
					);
					let counterclockwise_read = index_offset(
						&index_testimony.index,
						game_state.total_villagers(),
						i,
						false,
					);

					let clockwise_theoretical = &theoreticals[clockwise_read.0];
					let counterclockwise_theoretical = &theoreticals[counterclockwise_read.0];

					// I fucking hate wretch, they make everything difficult
					let mut found_unknown_good_clockwise = false;
					let mut found_unknown_good_counterclockwise = false;

					let mut clockwise_appears_evil =
						if clockwise_theoretical.unknown_unrevealed_good() {
							found_unknown_good_clockwise = true;
							(*direction == Direction::Clockwise) == *truthful
						} else {
							clockwise_theoretical.inner.true_identity().appears_evil()
						};
					let mut counterclockwise_appears_evil =
						if counterclockwise_theoretical.unknown_unrevealed_good() {
							found_unknown_good_counterclockwise = true;
							(*direction == Direction::CounterClockwise) == *truthful
						} else {
							counterclockwise_theoretical
								.inner
								.true_identity()
								.appears_evil()
						};

					// my head hurts
					if ((clockwise_appears_evil || found_unknown_good_clockwise)
						&& (counterclockwise_appears_evil || found_unknown_good_counterclockwise))
						&& *direction == Direction::Equidistant
						&& *truthful
					{
						clockwise_appears_evil = true;
						counterclockwise_appears_evil = true
					}

					if let Some(inner_expected_direction) =
						match (clockwise_appears_evil, counterclockwise_appears_evil) {
							(true, true) => Some(Direction::Equidistant),
							(true, false) => Some(Direction::Clockwise),
							(false, true) => Some(Direction::CounterClockwise),
							(false, false) => None,
						} {
						expected_direction = inner_expected_direction;
						break;
					}
				}

				*direction == expected_direction
			}
			Testimony::Knitter(evil_pairs_claim) => {
				let mut pairs_count = 0;
				let mut theoretical_pairs_count = 0;
				for i in 0..game_state.total_villagers() {
					let j = if i == (game_state.total_villagers() - 1) {
						0
					} else {
						i + 1
					};

					let left_theoretical = &theoreticals[i];
					let right_theoretical = &theoreticals[j];

					if left_theoretical.inner.true_identity().appears_evil()
						&& right_theoretical.inner.true_identity().appears_evil()
					{
						pairs_count += 1;
					} else if (left_theoretical.unknown_unrevealed_good()
						|| left_theoretical.inner.true_identity().appears_evil())
						&& (right_theoretical.unknown_unrevealed_good()
							|| right_theoretical.inner.true_identity().appears_evil())
					{
						theoretical_pairs_count += 1;
					}
				}

				pairs_count <= evil_pairs_claim.pairs()
					&& (pairs_count + theoretical_pairs_count) >= evil_pairs_claim.pairs()
			}
			Testimony::Bard(distance_option) => match distance_option {
				Some(distance) => {
					let bard_index = &index_testimony.index;

					let matches;
					let mut i = 0;
					loop {
						i += 1;
						let clockwise_read =
							index_offset(&bard_index, game_state.total_villagers(), i, true);
						let counterclockwise_read =
							index_offset(&bard_index, game_state.total_villagers(), i, false);

						let clockwise_theoretical = &theoreticals[clockwise_read.0];
						let counterclockwise_theoretical = &theoreticals[counterclockwise_read.0];

						let clockwise_corrupt = clockwise_theoretical.inner.corrupted();
						let counterclockwise_corrupt =
							counterclockwise_theoretical.inner.corrupted();
						if clockwise_corrupt || counterclockwise_corrupt {
							matches = i == *distance;
							break;
						}
					}

					matches
				}
				None => theoreticals
					.iter()
					.all(|theoretical| !theoretical.inner.corrupted()),
			},
		};

		let full_testimony = board_config.villagers[index_testimony.index.0]
			.inner
			.instance()
			.testimony()
			.as_ref()
			.unwrap();

		if testimony_valid != *truthful {
			debug!(logger: log, "Validation failed ({}: {}|FULL: {}): {}", if *truthful { "TRUE" } else { "FALSE" }, index_testimony, full_testimony, board_config.description);
			return false;
		}
	}

	debug!(logger: log, "Validation passed: {}", board_config.description);

	true
}

fn assignment_applies(theoretical: &TheoreticalVillager, testimony: &Testimony) -> bool {
	match theoretical.inner.instance().testimony() {
		Some(actual_testimony) => testimony_exists_in_expression(actual_testimony, testimony),
		None => false,
	}
}

fn testimony_exists_in_expression(
	expression: &Expression<Testimony>,
	testimony: &Testimony,
) -> bool {
	match expression {
		Expression::Leaf(leaf) => leaf == testimony,
		Expression::Not(expression) => testimony_exists_in_expression(expression, testimony),
		Expression::And(lhs, rhs) | Expression::Or(lhs, rhs) => {
			testimony_exists_in_expression(lhs, testimony)
				|| testimony_exists_in_expression(rhs, testimony)
		}
	}
}
