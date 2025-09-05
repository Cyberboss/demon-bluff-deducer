#![feature(breakpoint, cold_path, rust_cold_cc, hash_set_entry, gen_blocks)]

mod build_board_layouts;
mod build_expression_for_villager_set;
mod expression_assertion;
mod player_action;
mod prediction_error;
mod with_theoretical_testimony;

use core::panic;
use std::{
	arch::breakpoint,
	cmp::{max, min},
	collections::{BTreeMap, BTreeSet, HashMap, HashSet, hash_map::Entry},
	fs::File,
	str::FromStr,
	sync::atomic::{AtomicI32, Ordering},
	usize,
};

use build_board_layouts::{BoardLayout, TheoreticalVillager, build_board_layouts};
use build_expression_for_villager_set::{IndexTestimony, build_expression_for_villager_set};
use demon_bluff_gameplay_engine::{
	Expression,
	affect::Affect,
	game_state::{self, GameState},
	testimony::{self, ConfessorClaim, Testimony, index_offset},
	villager::{
		ConfirmedVillager, GoodVillager, Minion, Outcast, Villager, VillagerArchetype,
		VillagerIndex, VillagerInstance,
	},
};
use expression_assertion::{collect_satisfying_assignments, evaluate_with_assignment};
use log::{Log, debug, info, logger, warn};
use player_action::AbilityAttempt;
use rayon::prelude::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};
use with_theoretical_testimony::with_theoretical_testimony;

pub use self::{player_action::PlayerAction, prediction_error::PredictionError};

struct PredictionResult {
	all_matching_layouts: HashSet<BoardLayout>,
	board_layouts_by_similar_configs: HashMap<BTreeSet<VillagerIndex>, BTreeSet<BoardLayout>>,
	most_common_indicies: Option<(Vec<VillagerIndex>, Option<usize>)>,
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

	let mut need_more_info_result = None;
	if any_revealed {
		let initial_layouts = build_board_layouts(state);

		match predict_core(log, state, initial_layouts, false) {
			PredictionResult2::KillResult(hash_set) => {
				return hash_set;
			}
			PredictionResult2::NeedMoreInfoResult(hash_set) => {
				need_more_info_result = Some(hash_set)
			}
			PredictionResult2::ConfigCountsAfterAbility(_) => panic!("Incorrect return type!"),
		}
	}

	if state.night_actions_in_play()
		|| state
			.deck()
			.iter()
			.any(|archetype| *archetype == VillagerArchetype::Minion(Minion::Witch))
	{
		todo!("Need better revealing algorithm to support these cases")
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

	match hidden_index {
		Some(villager_to_reveal) => {
			let mut actions = HashSet::new();
			actions.insert(PlayerAction::TryReveal(villager_to_reveal));

			Ok(actions)
		}
		None => {
			info!(logger: log, "We must try to use an ability");

			let initial_layouts = need_more_info_result.expect("Udhfhfhfh");
			let layouts = with_theoretical_testimony(state, initial_layouts);

			// for each theoretical testimony find the group of

			let mut least_options: Option<(HashSet<AbilityAttempt>, usize)> = None;
			for (ability_attempt, predicted_layouts) in layouts {
				if let PredictionResult2::ConfigCountsAfterAbility(result) =
					predict_core(log, state, predicted_layouts, true)
				{
					let thing = match least_options {
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

					least_options = Some(thing);
				} else {
					panic!("Prediction was not allowed to return non and it did it anyway")
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
	NeedMoreInfoResult(HashSet<BoardLayout>),
}

enum PredictionResult3 {
	PredictionResult(PredictionResult),
	NeedMoreInfoResult(HashSet<BoardLayout>),
}

fn predict_core(
	log: &impl Log,
	state: &GameState,
	layouts: impl IntoIterator<Item = BoardLayout>,
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
						valid_prediction.all_matching_layouts,
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
	configs: impl IntoIterator<Item = BoardLayout>,
	allow_retry: bool,
) -> Result<PredictionResult3, PredictionError> {
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
			potential_board_configurations.iter().cloned().collect(),
		);

		return Ok(PredictionResult3::PredictionResult(PredictionResult {
			all_matching_layouts: potential_board_configurations.into_iter().collect(),
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

		let mut all_matching_layouts = HashSet::new();
		let mut matching_layouts = HashSet::new();

		let things_to_check: Vec<(
			usize,
			&Expression<IndexTestimony>,
			&HashMap<IndexTestimony, bool>,
		)> = potential_board_expressions
			.iter()
			.enumerate()
			.flat_map(|(index, board_expression)| {
				let vec: Vec<(
					usize,
					&Expression<IndexTestimony>,
					&HashMap<IndexTestimony, bool>,
				)> = potential_assignments
					.iter()
					.map(|assignment| (index, board_expression, assignment))
					.collect();
				vec
			})
			.collect();

		let matching_configs = AtomicI32::new(0);
		let iteration_result: Vec<BoardLayout> = things_to_check
			.into_par_iter()
			.filter_map(|(index, board_expression, assignment)| {
				if evaluate_with_assignment(&board_expression, &assignment)
					&& validate_assignment(
						log,
						assignment,
						&potential_board_configurations[index],
						game_state,
					) {
					matching_configs.fetch_add(1, Ordering::Relaxed);
					Some(potential_board_configurations[index].clone())
				} else {
					None
				}
			})
			.collect();

		for matching_board_config in iteration_result {
			matching_layouts.insert(matching_board_config.evil_locations.clone());
			all_matching_layouts.insert(matching_board_config);
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

		if matching_layouts.len() == 1 {
			let matching_layout = matching_layouts.into_iter().next().unwrap();
			let matching_configs = potential_board_configurations
				.into_iter()
				.filter(|board_config| board_config.evil_locations == matching_layout)
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
						can_get_more_information |= !hidden_villager.cant_reveal();
						return;
					}
					Villager::Confirmed(confirmed_villager) => {
						confirmed_villager.instance().testimony()
					}
				}
				.is_none()
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
			for config in all_matching_layouts.iter() {
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
			for config in &all_matching_layouts {
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

			return Ok(PredictionResult3::PredictionResult(PredictionResult {
				all_matching_layouts,
				board_layouts_by_similar_configs,
				most_common_indicies: Some((most_common_evil_index_occurrences, None)),
			}));
		}
	} else {
		info!(logger: log, "No villagers with testimonies, can't predict.");
		Ok(PredictionResult3::NeedMoreInfoResult(
			potential_board_configurations.into_iter().collect(),
		))
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
	/*
	if game_state.reveal_order().len() == 7
		&& board_config
			.evil_locations
			.contains(&VillagerIndex::number(5))
		&& board_config
			.evil_locations
			.contains(&VillagerIndex::number(6))
	{
		let json = serde_json::to_string_pretty(board_config).unwrap();
		std::fs::write("S:/workspace/demon-bluff-deducer/board_config.json", json).unwrap();
		panic!("Done")
	}*/

	let theoreticals = &board_config.villagers;
	for (index_testimony, truthful) in assignment {
		let testifier = &theoreticals[index_testimony.index.0];
		if !assignment_applies(testifier, &index_testimony.testimony) {
			continue;
		}

		let testimony_valid = match &index_testimony.testimony {
			Testimony::Good(villager_index) => !theoreticals[villager_index.0]
				.inner
				.true_identity()
				.is_evil(),
			Testimony::Evil(villager_index) => theoreticals[villager_index.0]
				.inner
				.true_identity()
				.appears_evil(),
			Testimony::Corrupt(villager_index) => theoreticals[villager_index.0].inner.corrupted(),
			Testimony::Lying(villager_index) => theoreticals[villager_index.0].inner.will_lie(),
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
