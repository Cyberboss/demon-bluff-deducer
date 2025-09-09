use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};

use demon_bluff_gameplay_engine::{
	Expression,
	game_state::GameState,
	testimony::{SlayResult, Testimony},
	villager::{GoodVillager, Outcast, VillagerArchetype, VillagerIndex},
};
use itertools::Itertools;
use log::Log;

use crate::{
	build_board_layouts::BoardLayout,
	build_expression_for_villager_set::{IndexTestimony, build_expression_for_villager_set},
	optimized_expression::OptimizedExpression,
	player_action::AbilityAttempt,
	validate_assignment,
};

pub fn with_theoretical_testimony(
	log: &impl Log,
	game_state: &GameState,
	board_configs: impl IntoIterator<Item = (BoardLayout, Vec<HashMap<IndexTestimony, bool>>)>,
) -> HashMap<AbilityAttempt, Vec<(BoardLayout, Vec<HashMap<IndexTestimony, bool>>)>> {
	let mut seen_ability_combinations = Vec::new();
	let mut current_ability_combination = HashSet::new();
	with_theoretical_testimony_inner(
		log,
		game_state,
		board_configs,
		&mut seen_ability_combinations,
		&mut current_ability_combination,
	)
}
pub fn with_theoretical_testimony_inner(
	log: &impl Log,
	game_state: &GameState,
	board_configs: impl IntoIterator<Item = (BoardLayout, Vec<HashMap<IndexTestimony, bool>>)>,
	seen_ability_combinations: &mut Vec<HashSet<AbilityAttempt>>,
	current_ability_combination: &mut HashSet<AbilityAttempt>,
) -> HashMap<AbilityAttempt, Vec<(BoardLayout, Vec<HashMap<IndexTestimony, bool>>)>> {
	let invalid_ability_attempts = seen_ability_combinations
		.iter()
		.filter_map(|attempt_set| {
			if attempt_set.len() != (current_ability_combination.len() + 1) {
				None
			} else {
				let mut the_one_we_dont_have = None;
				let missing_multiple =
					!current_ability_combination
						.iter()
						.all(|current_ability_attempt| {
							let contained = attempt_set.contains(current_ability_attempt);
							if contained {
								true
							} else if the_one_we_dont_have.is_some() {
								false
							} else {
								the_one_we_dont_have = Some(current_ability_attempt);
								true
							}
						});

				if missing_multiple {
					None
				} else {
					the_one_we_dont_have
				}
			}
		})
		.cloned()
		.collect();

	let board_configs_and_satisfying_assignments: Vec<(
		BoardLayout,
		Vec<HashMap<IndexTestimony, bool>>,
	)> = board_configs.into_iter().collect();
	let mut iterators = Vec::new();
	for (initial_board_config, valid_assignments) in board_configs_and_satisfying_assignments {
		iterators.push(iter_board_villagers_once(
			&initial_board_config,
			valid_assignments,
			&invalid_ability_attempts,
		));
	}

	let mut results: HashMap<
		AbilityAttempt,
		Vec<(BoardLayout, Vec<HashMap<IndexTestimony, bool>>)>,
	> = HashMap::with_capacity(iterators[0].len());
	loop {
		let mut group: Vec<(
			BoardLayout,
			AbilityAttempt,
			Vec<HashMap<IndexTestimony, bool>>,
		)> = Vec::new();
		for iterator in &mut iterators {
			if let Some(next_value) = iterator.pop_front() {
				if group.len() > 0 {
					assert_eq!(group[0].1, next_value.1);
				}

				group.push(next_value);
			}
		}

		if group.len() == 0 {
			break;
		}

		// the length of each iterator should be the same
		assert_eq!(group.len(), iterators.len());

		results.insert(
			group[0].1.clone(),
			group
				.into_iter()
				.map(|(layout, _, potential_assignments)| (layout, potential_assignments))
				.collect(),
		);
	}

	// recursively expand every solution until all testimonies are acquired
	let mut any_potential_testimonies_remaining = false;
	'outer: for (potential_layout, _) in results
		.iter()
		.flat_map(|(_, potential_layouts)| potential_layouts.iter())
	{
		for theoretical in &potential_layout.villagers {
			if theoretical.revealed && theoretical.inner.instance().testimony().is_none() {
				any_potential_testimonies_remaining = true;
				break 'outer;
			}
		}
	}

	if any_potential_testimonies_remaining {
		let wretch_in_play = game_state.role_in_play(VillagerArchetype::Outcast(Outcast::Wretch));
		let drunk_in_play = game_state.role_in_play(VillagerArchetype::Outcast(Outcast::Drunk));
		let knight_in_play =
			game_state.role_in_play(VillagerArchetype::GoodVillager(GoodVillager::Knight));
		let bombardier_in_play =
			game_state.role_in_play(VillagerArchetype::Outcast(Outcast::Bombardier));
		let mut expanded_results = HashMap::new();
		for (ability_attempt, new_layouts) in results {
			// to prevent exponential explosion, check layouts are satisfiable before recursing
			let valid_layouts_and_assignments: Vec<(
				BoardLayout,
				Vec<HashMap<IndexTestimony, bool>>,
			)> = new_layouts
				.into_iter()
				.filter_map(|(board_layout, mut potential_assignments)| {
					let expression = build_expression_for_villager_set(
						board_layout
							.villagers
							.iter()
							.map(|theoretical_villager| &theoretical_villager.inner),
					)
					.expect("There should be at least one testimony that we just built");

					let optimized_expression = OptimizedExpression::new(&expression);

					potential_assignments.retain(|map_assignment| {
						let mut assignment =
							Vec::with_capacity(optimized_expression.variables().len());
						for variable in optimized_expression.variables() {
							assignment.push(map_assignment[&variable]);
						}

						validate_assignment(
							log,
							&assignment,
							optimized_expression.variables(),
							&board_layout,
							game_state,
							wretch_in_play,
							drunk_in_play,
							knight_in_play,
							bombardier_in_play,
						)
					});

					if potential_assignments.is_empty() {
						None
					} else {
						Some((board_layout, potential_assignments))
					}
				})
				.collect();

			if !valid_layouts_and_assignments.is_empty() {
				current_ability_combination.insert(ability_attempt.clone());

				let expanded_layouts = with_theoretical_testimony_inner(
					log,
					game_state,
					valid_layouts_and_assignments,
					seen_ability_combinations,
					current_ability_combination,
				);
				let total_expanded_layouts = expanded_layouts
					.into_iter()
					.flat_map(|(_, expanded_layouts)| expanded_layouts.into_iter())
					.collect();

				current_ability_combination.remove(&ability_attempt);

				expanded_results.insert(ability_attempt, total_expanded_layouts);
			}
		}

		expanded_results
	} else {
		for (ability_attempt, _) in &results {
			let mut seen_combination = current_ability_combination.clone();
			seen_combination.reserve(1);
			seen_combination.insert(ability_attempt.clone());
			seen_ability_combinations.push(seen_combination);
		}

		results
	}
}

fn iter_board_villagers_once(
	inital_board_config: &BoardLayout,
	potential_assignments: Vec<HashMap<IndexTestimony, bool>>,
	invalid_ability_attempts: &HashSet<AbilityAttempt>,
) -> VecDeque<(
	BoardLayout,
	AbilityAttempt,
	Vec<HashMap<IndexTestimony, bool>>,
)> {
	let mut results = VecDeque::new();
	for (index, theoretical) in inital_board_config.villagers.iter().enumerate() {
		if theoretical.revealed
			&& let None = theoretical.inner.instance().testimony()
		{
			for (board_layout, ability_attempt, generated_testimonies) in theoretical_testimonies(
				&inital_board_config,
				VillagerIndex(index),
				invalid_ability_attempts,
			) {
				// TODO: avoid this clone on the last iteration somehow
				let mut potential_assignments = potential_assignments.clone();

				for generated_testimony in generated_testimonies {
					if !potential_assignments[0].contains_key(&generated_testimony) {
						// generate a positive and negative assignment set for each
						let initial_length = potential_assignments.len();
						potential_assignments.reserve(initial_length);
						for i in 0..initial_length {
							let assignment = &mut potential_assignments[i];
							let mut cloned_assignment = assignment.clone();
							assignment.insert(generated_testimony.clone(), true);

							cloned_assignment.insert(generated_testimony.clone(), false);
							potential_assignments.push(cloned_assignment);
						}
					}
				}

				results.push_back((board_layout, ability_attempt, potential_assignments));
			}

			break;
		}
	}

	results
}

gen fn theoretical_testimonies(
	board_config: &BoardLayout,
	testifier_index: VillagerIndex,
	invalid_ability_attempts: &HashSet<AbilityAttempt>,
) -> (BoardLayout, AbilityAttempt, Vec<IndexTestimony>) {
	let theoreticals = &board_config.villagers;
	let testifier = &theoreticals[testifier_index.0];
	let archetype = testifier.inner.instance().archetype();

	match archetype {
		VillagerArchetype::GoodVillager(good_villager) => match good_villager {
			GoodVillager::Alchemist => todo!("Alchemist testimony generation"),
			GoodVillager::Bard => todo!("Bard testimony generation"),
			GoodVillager::Bishop => todo!("Bishop testimony generation"),
			GoodVillager::Dreamer => todo!("Dreamer testimony generation"),
			GoodVillager::Druid => todo!("Druid testimony generation"),
			GoodVillager::FortuneTeller => {
				for index_combo in theoreticals
					.iter()
					.enumerate()
					.map(|(index, _)| VillagerIndex(index))
					.combinations(2)
				{
					let mut targets = BTreeSet::new();
					targets.extend(index_combo.iter().cloned());
					let ability_attempt = AbilityAttempt::new(testifier_index.clone(), targets);
					if invalid_ability_attempts.contains(&ability_attempt) {
						continue;
					}

					for expression in fortune_teller_expression(&index_combo) {
						let mut next_layout = board_config.clone();
						next_layout.description = format!(
							"{} - {} says {} or {} is{} evil",
							next_layout.description,
							testifier_index,
							index_combo[0],
							index_combo[1],
							if matches!(expression, Expression::Not(_)) {
								" NOT"
							} else {
								""
							}
						);

						let instance_to_modify = next_layout.villagers[testifier_index.0]
							.inner
							.instance_mut();

						instance_to_modify.set_testimony(expression);

						let mut testimonies = Vec::with_capacity(2);
						testimonies.extend(index_combo.iter().map(|index| {
							IndexTestimony::new(
								testifier_index.clone(),
								Testimony::Evil(index.clone()),
							)
						}));

						yield (next_layout, ability_attempt.clone(), testimonies);
					}
				}
			}
			GoodVillager::Jester => {
				for index_combo in theoreticals
					.iter()
					.enumerate()
					.map(|(index, _)| VillagerIndex(index))
					.combinations(3)
				{
					let mut targets = BTreeSet::new();
					targets.extend(index_combo.iter().cloned());
					let ability_attempt = AbilityAttempt::new(testifier_index.clone(), targets);
					if invalid_ability_attempts.contains(&ability_attempt) {
						continue;
					}

					for expression in jester_expression(&index_combo) {
						let mut next_layout = board_config.clone();
						next_layout.description = format!(
							"{} - {} says {}",
							next_layout.description, testifier_index, expression
						);

						let instance_to_modify = next_layout.villagers[testifier_index.0]
							.inner
							.instance_mut();

						instance_to_modify.set_testimony(expression);

						let mut testimonies = Vec::with_capacity(3);
						testimonies.extend(index_combo.iter().map(|index| {
							IndexTestimony::new(
								testifier_index.clone(),
								Testimony::Evil(index.clone()),
							)
						}));

						yield (next_layout, ability_attempt.clone(), testimonies);
					}
				}
			}
			GoodVillager::Judge => {
				for (index, _) in theoreticals.iter().enumerate() {
					let target_index = VillagerIndex(index);

					let mut targets = BTreeSet::new();
					targets.insert(target_index.clone());
					let ability_attempt = AbilityAttempt::new(testifier_index.clone(), targets);

					if invalid_ability_attempts.contains(&ability_attempt) {
						continue;
					}

					let raw_testimony = Testimony::Lying(target_index.clone());
					let base_expr = Expression::Leaf(raw_testimony.clone());

					let mut next_layout = board_config.clone();
					next_layout.villagers[testifier_index.0]
						.inner
						.instance_mut()
						.set_testimony(base_expr);

					let testimony_reference = next_layout.villagers[testifier_index.0]
						.inner
						.instance()
						.testimony()
						.as_ref()
						.unwrap();

					next_layout.description = format!(
						"{} - {} says {}",
						next_layout.description, testifier_index, testimony_reference
					);

					let negative_testimony = Expression::Not(Box::new(
						next_layout.villagers[testifier_index.0]
							.inner
							.instance()
							.testimony()
							.as_ref()
							.unwrap()
							.clone(),
					));

					let mut next_layout2 = board_config.clone();

					let index_testimony =
						IndexTestimony::new(testifier_index.clone(), raw_testimony);
					yield (
						next_layout,
						ability_attempt.clone(),
						vec![index_testimony.clone()],
					);

					next_layout2.villagers[testifier_index.0]
						.inner
						.instance_mut()
						.set_testimony(negative_testimony);

					let testimony_reference = next_layout2.villagers[testifier_index.0]
						.inner
						.instance()
						.testimony()
						.as_ref()
						.unwrap();

					next_layout2.description = format!(
						"{} - {} says {}",
						next_layout2.description, testifier_index, testimony_reference
					);

					yield (next_layout2, ability_attempt, vec![index_testimony]);
				}
			}
			GoodVillager::Slayer => {
				for (target_index, target_theoretical) in theoreticals
					.iter()
					.enumerate()
					.filter(move |(index, _)| *index != testifier_index.0)
				{
					let mut targets = BTreeSet::new();
					let target_index = VillagerIndex(target_index);
					targets.insert(target_index.clone());
					let ability_attempt = AbilityAttempt::new(testifier_index.clone(), targets);
					if invalid_ability_attempts.contains(&ability_attempt) {
						continue;
					}

					let mut next_layout = board_config.clone();

					let slayed;
					// TODO: The conditions for this may change. See https://discord.com/channels/1148903384968089640/1400926599628460052/1414305682290770012
					if testifier.inner.will_lie() {
						next_layout.description = format!(
							"{} - {} failed to slay {} due to lying",
							next_layout.description, testifier_index, target_index
						);
						slayed = false;
					} else if !target_theoretical.inner.true_identity().is_evil() {
						next_layout.description = format!(
							"{} - {} failed to slay {} due to them not being evil",
							next_layout.description, testifier_index, target_index
						);
						slayed = false;
					} else {
						next_layout.description = format!(
							"{} - {} slayed {}",
							next_layout.description, testifier_index, target_index
						);
						slayed = true;
						next_layout.villagers[target_index.0].actually_dead = true;
					}

					let testifier_instance_to_modify = next_layout.villagers[testifier_index.0]
						.inner
						.instance_mut();
					let raw_testimony = Testimony::Slayed(SlayResult::new(target_index, slayed));
					testifier_instance_to_modify
						.set_testimony(Expression::Leaf(raw_testimony.clone()));

					let mut testimonies = Vec::with_capacity(1);
					testimonies.push(IndexTestimony::new(testifier_index.clone(), raw_testimony));

					yield (next_layout, ability_attempt.clone(), testimonies);
				}
			}
			GoodVillager::Oracle => todo!("Oracle testimony generation"),
			GoodVillager::Poet => todo!("FUCKING POET TESTIMONY GENERATION!!!"),
			GoodVillager::Knitter => todo!("Knitter testimony generation"),
			GoodVillager::Architect
			| GoodVillager::Baker
			| GoodVillager::Confessor
			| GoodVillager::Empress
			| GoodVillager::Enlightened
			| GoodVillager::Gemcrafter
			| GoodVillager::Hunter
			| GoodVillager::Knight
			| GoodVillager::Lover
			| GoodVillager::Medium
			| GoodVillager::Scout
			| GoodVillager::Witness => panic!("A {} should not need its testimony generated!", archetype),
		},
		demon_bluff_gameplay_engine::villager::VillagerArchetype::Outcast(outcast) => match outcast
		{
			Outcast::Drunk | Outcast::Wretch | Outcast::Bombardier | Outcast::Doppelganger => {
				panic!("A {} should not have a testimony!", archetype)
			}
			Outcast::PlagueDoctor => {
				for (target_index, target_theoretical) in theoreticals
					.iter()
					.enumerate()
					.filter(move |(index, _)| *index != testifier_index.0)
				// PD always sees themselves as not corrupt, so this is uselesss
				{
					let mut targets = BTreeSet::new();
					let target_index = VillagerIndex(target_index);
					targets.insert(target_index.clone());
					let ability_attempt = AbilityAttempt::new(testifier_index.clone(), targets);
					if invalid_ability_attempts.contains(&ability_attempt) {
						continue;
					}

					let mut next_layout = board_config.clone();

					let truly_corrupt = target_theoretical.inner.corrupted();

					let says_corrupt = testifier.inner.will_lie() ^ truly_corrupt;
					let raw_testimony = Testimony::Corrupt(target_index.clone());
					if says_corrupt {
						next_layout.description = format!(
							"{} - {} {}correctly says: {} is corrupt",
							next_layout.description,
							testifier_index,
							if truly_corrupt { "" } else { "IN" },
							target_index
						);
					} else {
						next_layout.description = format!(
							"{} - {} {}correctly says: {} is NOT corrupt",
							next_layout.description,
							testifier_index,
							if truly_corrupt { "IN" } else { "" },
							target_index
						);
					}

					let mut testimonies = Vec::with_capacity(1);
					testimonies.push(IndexTestimony::new(
						testifier_index.clone(),
						raw_testimony.clone(),
					));

					if !says_corrupt {
						let testifier_instance_to_modify = next_layout.villagers[testifier_index.0]
							.inner
							.instance_mut();
						testifier_instance_to_modify.set_testimony(Expression::Not(Box::new(
							Expression::Leaf(raw_testimony),
						)));
						yield (next_layout, ability_attempt.clone(), testimonies);
						return;
					}

					for (index, _) in next_layout.villagers.into_iter().enumerate().filter(
						move |(index, villager)| {
							*index != testifier_index.0
								&& testifier.inner.will_lie()
									== !villager.inner.true_identity().appears_evil()
						},
					) {
						let evil_index = VillagerIndex(index);
						let next_description =
							format!("{}, {} is evil", next_layout.description, evil_index);
						let mut next_layout = board_config.clone();
						next_layout.description = next_description;
						let testifier_instance_to_modify = next_layout.villagers[testifier_index.0]
							.inner
							.instance_mut();
						testifier_instance_to_modify.set_testimony(Expression::And(
							Box::new(Expression::Leaf(raw_testimony.clone())),
							Box::new(Expression::Leaf(Testimony::Evil(evil_index))),
						));
						yield (next_layout, ability_attempt.clone(), testimonies.clone());
					}
				}
			}
		},
		VillagerArchetype::Minion(_) | VillagerArchetype::Demon(_) => {
			panic!("A {} should not have a testimony!", archetype)
		}
	}
}

fn jester_expression(indexes: &Vec<VillagerIndex>) -> [Expression<Testimony>; 4] {
	[
		Testimony::jester(
			indexes
				.as_slice()
				.try_into()
				.expect("Invalid number of indexes for a jester expression"),
			0,
		),
		Testimony::jester(indexes.as_slice().try_into().unwrap(), 1),
		Testimony::jester(indexes.as_slice().try_into().unwrap(), 2),
		Testimony::jester(indexes.as_slice().try_into().unwrap(), 3),
	]
}

fn fortune_teller_expression(indexes: &Vec<VillagerIndex>) -> [Expression<Testimony>; 2] {
	[
		Testimony::fortune_teller(
			indexes
				.as_slice()
				.try_into()
				.expect("Invalid number of indexes for a jester expression"),
			false,
		),
		Testimony::fortune_teller(indexes.as_slice().try_into().unwrap(), true),
	]
}
