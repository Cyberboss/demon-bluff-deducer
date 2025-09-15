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
	clone,
	cmp::max,
	collections::{BTreeSet, HashMap, HashSet, hash_map::Entry},
	fmt::format,
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
use log::{Level, Log, debug, info, log_enabled, trace, warn};
use optimized_expression::OptimizedExpression;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use with_theoretical_testimony::{LayoutWithTestimonyAssigments, with_theoretical_testimony};

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
			PredictionResult2::NeedMoreInfoResult(layouts_with_assignments) => {
				need_more_info_result = Some(layouts_with_assignments)
			}
			PredictionResult2::ConfigCountsAfterAbility(_) => {
				unreachable!("Incorrect return type!")
			}
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

			let mut initial_problem_space = HashMap::new();
			for layout in &initial_layouts {
				match initial_problem_space.entry(layout.layout.evil_locations.clone()) {
					Entry::Occupied(mut occupied_entry) => {
						let new_value = occupied_entry.get() + 1;
						occupied_entry.insert(new_value);
					}
					Entry::Vacant(vacant_entry) => {
						vacant_entry.insert(1);
					}
				}
			}

			let mut layouts = with_theoretical_testimony(log, state, &initial_layouts);

			let mut attempt_order = Vec::with_capacity(layouts.attempt_predictions.len());
			attempt_order.extend(
				layouts
					.attempt_predictions
					.iter()
					.map(|(ability_attempt, _)| ability_attempt)
					.cloned(),
			);
			attempt_order.sort();

			let mut previously_caluclated_attempts = Vec::with_capacity(attempt_order.len());

			// evaluating every ability combination is too damn expensive (and impossible to empiracally verify in the game)
			// order here wants to be deterministic for testing purposes, so collect and sort the keys

			// for each theoretical testimony find the group of
			let mut abilities_with_highest_factor_of_single_evil_layout: Option<(
				HashSet<AbilityAttempt>,
				HashSet<BTreeSet<VillagerIndex>>,
				f64,
				usize,
			)> = None;
			let mut invalidated_evil_layouts = HashSet::new();
			'outer: loop {
				for (ability_attempt_index, ability_attempt) in attempt_order.iter().enumerate() {
					let mutation_option =
						layouts.attempt_predictions.remove_entry(&ability_attempt);

					if previously_caluclated_attempts.len() <= ability_attempt_index {
						debug_assert_eq!(
							ability_attempt_index,
							previously_caluclated_attempts.len()
						);

						let (_, mutations) = mutation_option.expect("Impossble");

						let total_layouts_in_mutations: usize = mutations
							.iter()
							.map(|mutation| mutation.potential_layouts.len())
							.sum();

						info!(logger: log, "Theorizing ({} board layouts): {}", total_layouts_in_mutations, ability_attempt);

						if let PredictionResult2::ConfigCountsAfterAbility(
							layout_counts_from_prediction,
						) = predict_core(
							log,
							state,
							mutations
								.iter()
								.flat_map(|mutation| mutation.potential_layouts.iter())
								.cloned()
								.map(|potential_layout| {
									(
										potential_layout.layout,
										Some(potential_layout.satisfying_assignments),
									)
								}),
							true,
						) {
							previously_caluclated_attempts.push(layout_counts_from_prediction);
						} else {
							unreachable!(
								"Prediction was not allowed to return non and it did it anyway"
							);
						}
					}

					let layout_counts = &previously_caluclated_attempts[ability_attempt_index];

					let mut highest_entry = 0;
					let mut total_board_layouts = 0;
					let mut highest_evils_layout = None;

					let mut mutually_exclusive_layouts: HashMap<
						&Expression<Testimony>,
						Vec<&BTreeSet<VillagerIndex>>,
					> = HashMap::new();
					let layout_counts_len = layout_counts.len();

					for (layouts, evils_layout) in layout_counts {
						if invalidated_evil_layouts.contains(evils_layout) {
							continue;
						}

						let layout_count = layouts.len();

						if layout_count == 0 {
							if let Some((_, contributing_evil_layouts, _, _)) =
								abilities_with_highest_factor_of_single_evil_layout.as_mut()
								&& contributing_evil_layouts.contains(&evils_layout)
							{
								// need to recalculate everything now since we could have just invalidated the best factor
								abilities_with_highest_factor_of_single_evil_layout = None;
								invalidated_evil_layouts.insert(evils_layout.clone());
								continue 'outer;
							}

							invalidated_evil_layouts.insert(evils_layout.clone());
							continue;
						}

						if layout_count > highest_entry {
							highest_entry = layout_count;
							highest_evils_layout = Some(evils_layout);
						}

						total_board_layouts += layout_count;
						for theorized_layout in layouts {
							debug_assert_eq!(theorized_layout.evil_locations, *evils_layout);
							let generated_testimony = theorized_layout.villagers
								[ability_attempt.source().0]
								.inner
								.instance()
								.testimony()
								.as_ref()
								.expect("Ability usage didn't generate testimony?");
							match mutually_exclusive_layouts.entry(generated_testimony) {
								Entry::Occupied(mut occupied_entry) => {
									let vec = occupied_entry.get_mut();
									vec.push(&theorized_layout.evil_locations);
								}
								Entry::Vacant(vacant_entry) => {
									let vec = vec![&theorized_layout.evil_locations];
									vacant_entry.insert(vec);
								}
							}
						}
					}

					let mut mutually_exclusive_groups: Vec<HashSet<&BTreeSet<VillagerIndex>>> =
						Vec::new();

					for (_, mut mutually_exclusive_layout) in mutually_exclusive_layouts {
						mutually_exclusive_groups.retain(|previous_group| {
							let mut any_overlap = false;
							for i in 0..mutually_exclusive_layout.len() {
								let layout = mutually_exclusive_layout[i];
								let overlap = previous_group.contains(layout);
								if overlap {
									mutually_exclusive_layout.extend(previous_group.iter());
									any_overlap = true;
								}
							}

							!any_overlap
						});

						mutually_exclusive_groups
							.push(mutually_exclusive_layout.into_iter().collect());
					}

					let factor = highest_entry as f64 / total_board_layouts as f64;
					let highest_evils_layout = highest_evils_layout.expect("sign bruh");

					let problem_space_reduction = mutually_exclusive_groups.len();

					info!(
						"Theory resulted in {} possible evil configurations factor: {} ({} ({}) / {}). PSR: {}",
						layout_counts_len,
						factor,
						highest_entry,
						highest_evils_layout
							.iter()
							.map(|index| format!("{}", index))
							.join("|"),
						total_board_layouts,
						problem_space_reduction
					);

					let (
						ability_uses,
						new_contributing_evil_layouts,
						evil_location_configurations_reduction,
						new_problem_space_reduction,
					) = match abilities_with_highest_factor_of_single_evil_layout {
						Some((
							mut old_highest_factor_abilities,
							mut contributing_evil_layouts,
							old_highest_factor,
							old_problem_space_reduction,
						)) => {
							if old_highest_factor < factor
								|| (old_highest_factor == factor
									&& old_problem_space_reduction < problem_space_reduction)
							{
								let mut new_highest_factor_abilities = HashSet::new();
								new_highest_factor_abilities.insert(ability_attempt.clone());
								contributing_evil_layouts.insert(highest_evils_layout.clone());
								(
									new_highest_factor_abilities,
									contributing_evil_layouts,
									factor,
									problem_space_reduction,
								)
							} else {
								if old_highest_factor == factor
									&& old_problem_space_reduction == problem_space_reduction
								{
									old_highest_factor_abilities.insert(ability_attempt.clone());
									contributing_evil_layouts.insert(highest_evils_layout.clone());
								}

								(
									old_highest_factor_abilities,
									contributing_evil_layouts,
									old_highest_factor,
									old_problem_space_reduction,
								)
							}
						}
						None => {
							let mut new_highest_factor_abilities = HashSet::new();
							let mut new_contributing_evil_layouts = HashSet::new();
							new_highest_factor_abilities.insert(ability_attempt.clone());
							new_contributing_evil_layouts.insert(highest_evils_layout.clone());
							(
								new_highest_factor_abilities,
								new_contributing_evil_layouts,
								factor,
								problem_space_reduction,
							)
						}
					};

					// optimization, take the first result that gives us one layout
					let this_one_works = factor == 1.0;

					abilities_with_highest_factor_of_single_evil_layout = Some((
						ability_uses,
						new_contributing_evil_layouts,
						evil_location_configurations_reduction,
						new_problem_space_reduction,
					));

					if this_one_works {
						info!(logger: log, "Found an ability path that leads to a single evil configuration taking it and earlying out on theorizing");
						break;
					}
				}

				break;
			}

			let (ability_attempts, _, factor, problem_space_reduction) =
				abilities_with_highest_factor_of_single_evil_layout
					.expect("No value ability usages found??");

			let mut attempt_strings: Vec<String> = ability_attempts
				.iter()
				.map(|attempt| format!("{}", attempt))
				.collect();
			attempt_strings.sort();
			info!(logger: log, "Selecting the path of \"{}\" which has a factor of {} of layouts being equivalent to one evil layout (PSR: {}). {} invalid layouts were eliminated", attempt_strings.join("|"), factor, problem_space_reduction, invalidated_evil_layouts.len());

			Ok(ability_attempts
				.into_iter()
				.map(|ability_attempt| PlayerAction::Ability(ability_attempt))
				.collect())
		}
	}
}

enum PredictionResult2 {
	KillResult(Result<HashSet<PlayerAction>, PredictionError>),
	ConfigCountsAfterAbility(Vec<(Vec<BoardLayout>, BTreeSet<VillagerIndex>)>),
	NeedMoreInfoResult(Vec<LayoutWithTestimonyAssigments>),
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
	let mut initial_evil_layouts = HashSet::new();
	let prediction_result = predict_board_configs(
		log,
		state,
		layouts.map(|(layout, testimonies)| {
			initial_evil_layouts.insert(layout.evil_locations.clone());
			(layout, testimonies)
		}),
		!count_configs,
	);
	match prediction_result {
		Ok(valid_prediction) => {
			match valid_prediction {
				PredictionResult3::PredictionResult(valid_prediction) => {
					if count_configs {
						// we actually want to eliminate board layouts that have narrowed things down to the remaining evils

						let result = PredictionResult2::ConfigCountsAfterAbility(
							initial_evil_layouts
								.into_iter()
								.map(|evils_layout| {
									let board_layouts = valid_prediction
										.board_layouts_by_similar_configs
										.get(&evils_layout)
										.map(|layouts| layouts.iter().cloned().collect())
										.unwrap_or(Vec::new());
									(board_layouts, evils_layout)
								})
								.collect(),
						);

						return result;
					}

					if let Some((most_common_indicies, _)) = valid_prediction.most_common_indicies {
						let mut actions: HashSet<PlayerAction> =
							HashSet::with_capacity(most_common_indicies.len());
						// select the most common indicies
						for index in most_common_indicies {
							// TODO: does it make sense to kill ASAP if there's a night affect in play?
							actions.insert(PlayerAction::TryExecute(index));
						}

						debug_assert_ne!(0, actions.len());

						return PredictionResult2::KillResult(Ok(actions));
					}

					debug_assert_eq!(
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
					PredictionResult2::NeedMoreInfoResult(
						hash_set
							.into_iter()
							.map(
								|(layout, satisfying_assignments)| LayoutWithTestimonyAssigments {
									layout,
									satisfying_assignments,
								},
							)
							.collect(),
					)
				}
			}
		}
		Err(prediction_error) => PredictionResult2::KillResult(Err(prediction_error)),
	}
}

struct PreviousAssignments<'a> {
	matching_board_index: usize,
	previous_assignments: &'a Vec<HashMap<IndexTestimony, bool>>,
}

enum AssignmentsType<'a> {
	All(Vec<bool>),
	Previous(PreviousAssignments<'a>),
}

enum ExpandedAssignmentsType<'a> {
	All(&'a Vec<bool>),
	Previous(&'a HashMap<IndexTestimony, bool>),
}

fn predict_board_configs(
	log: &impl Log,
	game_state: &GameState,
	configs: impl Iterator<Item = (BoardLayout, Option<Vec<HashMap<IndexTestimony, bool>>>)>,
	non_hypothetical_pass: bool,
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
		potential_board_expressions.push(config_expression);
	}

	if potential_board_expressions.iter().next().is_some() {
		let optimized_expressions: Vec<OptimizedExpression<IndexTestimony>> =
			potential_board_expressions
				.iter()
				.map(|board_expression| OptimizedExpression::new(&board_expression))
				.collect();
		let master_expression = Expression::MajorOr(potential_board_expressions);
		let optimized_master_expression = OptimizedExpression::new(&master_expression);

		let master_expression_satisfying_assignments: Vec<AssignmentsType> =
			if non_hypothetical_pass {
				collect_satisfying_assignments(&optimized_master_expression)
					.into_iter()
					.map(|assignment| AssignmentsType::All(assignment))
					.collect()
			} else {
				// all board configurations should have potential assignments alongside them and they should NOT overlap
				// convert them to optimized form
				potential_board_configurations
					.iter()
					.enumerate()
					.map(|(board_index, (_, previously_satisfying_assignments))| {
						let assignment =
							previously_satisfying_assignments
								.as_ref()
								.unwrap_or_else(|| {
									unreachable!("We should have potential assignments predicted!")
								});

						AssignmentsType::Previous(PreviousAssignments {
							matching_board_index: board_index,
							previous_assignments: assignment,
						})
					})
					.collect()
			};

		if master_expression_satisfying_assignments.is_empty() {
			return Err(PredictionError::GameUnsolvable);
		}

		info!(
			logger: log,
			"{} potential assignments to evaluate",
			master_expression_satisfying_assignments.len()
		);
		let mut all_matching_layouts: HashMap<BoardLayout, Vec<HashMap<IndexTestimony, bool>>> =
			HashMap::new();
		let mut matching_layouts = HashSet::new();

		let assignments_to_iterate: Vec<(usize, ExpandedAssignmentsType)> =
			match &master_expression_satisfying_assignments[0] {
				// check every assignment against every board
				AssignmentsType::All(_) => optimized_expressions
					.iter()
					.enumerate()
					.flat_map(|(board_index, _)| {
						master_expression_satisfying_assignments.iter().map(
							move |satisfying_assignment| {
								(
									board_index,
									match satisfying_assignment {
										AssignmentsType::All(items) => {
											ExpandedAssignmentsType::All(items)
										}
										AssignmentsType::Previous(_) => {
											unreachable!(
												"Index 0 was AssignmentsType::All but another index was AssignmentsType::Previous??"
											)
										}
									},
								)
							},
						)
					})
					.collect(),
				// check only previously satisfying assignments against their source boards
				AssignmentsType::Previous(_) => master_expression_satisfying_assignments
					.into_iter()
					.flat_map(|assignment_type| match assignment_type {
						AssignmentsType::All(_) => unreachable!(
							"Index 0 was AssignmentsType::Previous but another index was AssignmentsType::All??"
						),
						AssignmentsType::Previous(previous_assignments) => previous_assignments
							.previous_assignments
							.into_iter()
							.map(move |assignment_map| {
								(
									previous_assignments.matching_board_index,
									ExpandedAssignmentsType::Previous(assignment_map),
								)
							}),
					})
					.collect(),
			};

		let matching_configs = AtomicI32::new(0);
		let wretch_in_play = game_state.role_in_play(VillagerArchetype::Outcast(Outcast::Wretch));
		let drunk_in_play = game_state.role_in_play(VillagerArchetype::Outcast(Outcast::Drunk));
		let knight_in_play =
			game_state.role_in_play(VillagerArchetype::GoodVillager(GoodVillager::Knight));
		let bombardier_in_play =
			game_state.role_in_play(VillagerArchetype::Outcast(Outcast::Bombardier));

		let board_index_satisfying_assignments: Vec<(usize, HashMap<IndexTestimony, bool>)> =
			assignments_to_iterate
				.into_par_iter()
				.filter_map(|(board_index, assignment_type)| {
					let mapped_assignment;
					let board_expression = &optimized_expressions[board_index];
					let assignment = match assignment_type {
						ExpandedAssignmentsType::All(items) => items,
						ExpandedAssignmentsType::Previous(previous_assignments) => {
							let mut assignment_vec =
								Vec::with_capacity(board_expression.variables().len());

							for variable in board_expression.variables().iter() {
								if let Some(testimony_trutfulness) =
									previous_assignments.get(variable)
								{
									assignment_vec.push(*testimony_trutfulness)
								}
							}

							mapped_assignment = assignment_vec;
							&mapped_assignment
						}
					};

					let (layout, _) = &potential_board_configurations[board_index];

					if board_expression.satisfies(|variable_index| assignment[variable_index])
						&& validate_assignment(
							log,
							&assignment,
							board_expression.variables(),
							layout,
							game_state,
							wretch_in_play,
							drunk_in_play,
							knight_in_play,
							bombardier_in_play,
						) {
						matching_configs.fetch_add(1, Ordering::Relaxed);

						let satisfying_assignment = match assignment_type {
							ExpandedAssignmentsType::All(_) => {
								let mut satisfying_assignment_builder =
									HashMap::with_capacity(board_expression.variables().len());
								for (index, variable) in
									board_expression.variables().iter().enumerate()
								{
									satisfying_assignment_builder
										.insert(variable.clone(), assignment[index]);
								}

								satisfying_assignment_builder
							}
							ExpandedAssignmentsType::Previous(previous_assignments) => {
								previous_assignments.clone()
							}
						};
						Some((board_index, satisfying_assignment))
					} else {
						None
					}
				})
				.collect();

		for (matching_board_config_index, satisfying_assignment) in
			board_index_satisfying_assignments
		{
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

		if log_enabled!(logger: log, Level::Info) {
			let mut evil_layout_count = 0;
			let mut layout_count = 0;
			let matching_configs = matching_configs.fetch_add(0, Ordering::Acquire);
			info!(logger: log, "Filtered to {} evil layouts amongst {} configurations", matching_layouts.len(), matching_configs);
			for evil_locations in &matching_layouts {
				evil_layout_count += 1;
				info!(logger: log, "Potential Evils Layout {}", evil_layout_count);
				for index in evil_locations {
					info!(logger: log, "- {}", index);
				}
				let mut iterator = all_matching_layouts
					.iter()
					.filter(|(layout, _)| layout.evil_locations == *evil_locations);
				if log_enabled!(logger: log, Level::Debug) {
					debug!("Instances:");
					for (matching_layout, _) in iterator {
						layout_count += 1;
						debug!(logger: log, "Layout {}: {}", layout_count, matching_layout.description);
					}
				} else {
					info!("EX: {}", iterator.next().unwrap().0.description);
				}
			}
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

		if non_hypothetical_pass {
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

		if most_common_evil_index_occurrences.len() == 1
			&& *(evil_index_occurrences_in_matching_layouts
				.get(&most_common_evil_index_occurrences[0])
				.unwrap()) == matching_layouts.len()
		{
			let most_common_index = &most_common_evil_index_occurrences[0];
			info!(logger: log, "We found the an evil that all layouts share: {}. BEND HIM!", most_common_index);
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
			if non_hypothetical_pass {
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

			if non_hypothetical_pass {
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
	debug_assert_eq!(variables.len(), assignment.len());

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
					theoretical.inner.true_identity() == role_claim.role()
					// WREEEEETTTTCCHHHH
					|| (matches!(role_claim.role(), VillagerArchetype::Minion(_)) && theoretical.inner.true_identity().appears_evil() && !theoretical.inner.true_identity().is_evil()),
					!role_claim.role().is_evil(),
				)
			}
			Testimony::Invincible(villager_index) => {
				let theoretical = &theoreticals[villager_index.0];

				if_unknown_good_use_truthful(
					theoretical,
					*theoretical.inner.true_identity()
						== VillagerArchetype::GoodVillager(GoodVillager::Knight)
						&& !theoretical.inner.will_lie(),
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
						&theoreticals[slay_result.index().0],
						!confirmed_target.true_identity().appears_evil(),
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
						if i > distance.get() {
							matches = false;
							break;
						}

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
							matches = i == distance.get();
							break;
						}
					}

					matches
				}
				None => theoreticals
					.iter()
					.all(|theoretical| !theoretical.inner.corrupted()),
			},
			Testimony::FortuneTeller(fortune_teller_claim) => {
				let targets = fortune_teller_claim.targets();
				let thing_1 = &theoreticals[targets[0].0];
				let thing_2 = &theoreticals[targets[1].0];
				let correct_1 = if_unknown_good_use_truthful(
					thing_1,
					thing_1.inner.true_identity().appears_evil(),
					true,
				);
				let correct_2 = if_unknown_good_use_truthful(
					thing_2,
					thing_2.inner.true_identity().appears_evil(),
					true,
				);
				if fortune_teller_claim.evil() {
					correct_1 || correct_2
				} else {
					!correct_1 && !correct_2
				}
			}
			Testimony::Druid(druid_claim) => match druid_claim.outcast() {
				Some(minion) => {
					let mut found_match = false;
					for target in druid_claim.targets() {
						let target_theoretical = &theoreticals[target.0];
						found_match |= if_unknown_good_use_truthful(
							target_theoretical,
							*target_theoretical.inner.true_identity()
								== VillagerArchetype::Outcast(minion.clone()),
							true,
						);
					}

					found_match
				}
				None => {
					let mut found_match = false;
					for target in druid_claim.targets() {
						found_match |= matches!(
							theoreticals[target.0].inner.true_identity(),
							VillagerArchetype::Outcast(_)
						);
					}

					!found_match
				}
			},
		};

		let full_testimony = board_config.villagers[index_testimony.index.0]
			.inner
			.instance()
			.testimony()
			.as_ref()
			.unwrap();

		if testimony_valid != *truthful {
			trace!(logger: log, "Validation failed ({}: {}|FULL: {}): {}", if *truthful { "TRUE" } else { "FALSE" }, index_testimony, full_testimony, board_config.description);
			return false;
		}
	}
	trace!(logger: log, "Validation passed: {}", board_config.description);

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
		Expression::MajorOr(expressions) => {
			for expression in expressions {
				if testimony_exists_in_expression(expression, testimony) {
					return true;
				}
			}

			false
		}
	}
}
