use std::collections::{BTreeSet, HashMap, hash_map::Entry};

use demon_bluff_gameplay_engine::{
	Expression,
	game_state::GameState,
	testimony::{FortuneTellerClaim, SlayResult, Testimony},
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

/* we have BoardLayouts A B C and D

ability 1:
- validates for board layout a in case x
- eliminates b
- validates for board layout c in case x and y
- validates for d in case y and z
ability 2:
- validates for layout a in case y,
- validates for layout b in case x, y, z
- eliminates c
- validates for d in case x

first things first, if any ability eliminates a layout completely, that layout IS NOT VALID, discard it across all hypotheticals


leaves us with a and d, how do we decide which to pick?

Ab1: A=2 D=3, Ab2: A=1, D=2

Ok thank you Merina, the one that should be picked is the one that provides the biggest fraction/percentage for a single decision
So Ab2 in this case for the 2/3 chance for D

*/

pub struct AbilityPrediction {
	pub attempt_predictions: HashMap<AbilityAttempt, Vec<PostAbilityBoardMutation>>,
}

pub struct PostAbilityBoardMutation {
	pub original_layout: BoardLayout,
	pub potential_layouts: Vec<LayoutWithTestimonyAssigments>,
}

#[derive(Debug, Clone)]
pub struct LayoutWithTestimonyAssigments {
	pub layout: BoardLayout,
	pub satisfying_assignments: Vec<HashMap<IndexTestimony, bool>>,
}

pub fn with_theoretical_testimony(
	log: &impl Log,
	game_state: &GameState,
	board_configs_and_satisfying_assignments: &Vec<LayoutWithTestimonyAssigments>,
) -> AbilityPrediction {
	let mut results: HashMap<AbilityAttempt, Vec<PostAbilityBoardMutation>> = HashMap::new();

	for (original_layout, mutation_vec) in
		board_configs_and_satisfying_assignments
			.iter()
			.map(|layout_with_testimony_assigments| {
				let local_results: Vec<(AbilityAttempt, LayoutWithTestimonyAssigments)> =
					generate_theoreticals_for_first_villager_with_ability(
						layout_with_testimony_assigments,
					)
					.collect();
				(&layout_with_testimony_assigments.layout, local_results)
			}) {
		for (ability_attempt, theoretical_layout) in mutation_vec {
			match results.entry(ability_attempt) {
				Entry::Occupied(mut occupied_entry) => {
					let mutations_vec: &mut Vec<PostAbilityBoardMutation> =
						occupied_entry.get_mut();
					let matching_mutation = mutations_vec
						.iter_mut()
						.filter(|existing_mutation| {
							existing_mutation.original_layout == *original_layout
						})
						.next();

					match matching_mutation {
						Some(existing_mutation) => {
							existing_mutation.potential_layouts.push(theoretical_layout);
						}
						None => {
							let mutation = PostAbilityBoardMutation {
								original_layout: original_layout.clone(),
								potential_layouts: vec![theoretical_layout],
							};
							mutations_vec.push(mutation);
						}
					}
				}
				Entry::Vacant(vacant_entry) => {
					let mutation = PostAbilityBoardMutation {
						original_layout: original_layout.clone(),
						potential_layouts: vec![theoretical_layout],
					};
					vacant_entry.insert(vec![mutation]);
				}
			}
		}
	}

	// recursively expand every solution until all testimonies are acquired
	let mut any_potential_testimonies_remaining = false;
	'outer: for potential_mutation in results
		.iter()
		.flat_map(|(_, potential_mutations)| potential_mutations.iter())
	{
		for theoretical in &potential_mutation.potential_layouts[0].layout.villagers {
			if theoretical.revealed && theoretical.inner.instance().testimony().is_none() {
				any_potential_testimonies_remaining = true;
				break 'outer;
			}
		}
	}

	let final_results = if any_potential_testimonies_remaining {
		let wretch_in_play = game_state.role_in_play(VillagerArchetype::Outcast(Outcast::Wretch));
		let drunk_in_play = game_state.role_in_play(VillagerArchetype::Outcast(Outcast::Drunk));
		let knight_in_play =
			game_state.role_in_play(VillagerArchetype::GoodVillager(GoodVillager::Knight));
		let bombardier_in_play =
			game_state.role_in_play(VillagerArchetype::Outcast(Outcast::Bombardier));
		let mut expanded_results: HashMap<AbilityAttempt, Vec<PostAbilityBoardMutation>> =
			HashMap::new();
		for (ability_attempt, original_mutations) in results {
			for original_mutation in original_mutations {
				// to prevent exponential explosion, check layouts are satisfiable before recursing
				let valid_layouts_and_assignments: Vec<LayoutWithTestimonyAssigments> =
					original_mutation
						.potential_layouts
						.into_iter()
						.filter_map(|mut new_layout| {
							let expression = build_expression_for_villager_set(
								new_layout
									.layout
									.villagers
									.iter()
									.map(|theoretical_villager| &theoretical_villager.inner),
							)
							.expect("There should be at least one testimony that we just built");

							let optimized_expression = OptimizedExpression::new(&expression);

							new_layout.satisfying_assignments.retain(|map_assignment| {
								let mut assignment =
									Vec::with_capacity(optimized_expression.variables().len());
								for variable in optimized_expression.variables() {
									// error here means testimonies from theoretical weren't generated properly
									assignment.push(map_assignment[&variable]);
								}

								validate_assignment(
									log,
									&assignment,
									optimized_expression.variables(),
									&new_layout.layout,
									game_state,
									wretch_in_play,
									drunk_in_play,
									knight_in_play,
									bombardier_in_play,
								)
							});

							if new_layout.satisfying_assignments.is_empty() {
								None
							} else {
								Some(new_layout)
							}
						})
						.collect();

				if !valid_layouts_and_assignments.is_empty() {
					let expanded_theoreticals =
						with_theoretical_testimony(log, game_state, &valid_layouts_and_assignments);
					let total_expanded_layouts = expanded_theoreticals
						.attempt_predictions
						.into_iter()
						.flat_map(|(_, expanded_mutations)| expanded_mutations)
						.flat_map(|expanded_mutation| expanded_mutation.potential_layouts)
						.collect();

					let new_mutation = PostAbilityBoardMutation {
						original_layout: original_mutation.original_layout,
						potential_layouts: total_expanded_layouts,
					};

					match expanded_results.entry(ability_attempt.clone()) {
						Entry::Occupied(mut occupied_entry) => {
							occupied_entry.get_mut().push(new_mutation)
						}
						Entry::Vacant(vacant_entry) => {
							vacant_entry.insert(vec![new_mutation]);
						}
					}
				}
			}
		}

		expanded_results
	} else {
		results
	};

	AbilityPrediction {
		attempt_predictions: final_results,
	}
}

gen fn generate_theoreticals_for_first_villager_with_ability(
	original_layout_with_testimonies: &LayoutWithTestimonyAssigments,
) -> (AbilityAttempt, LayoutWithTestimonyAssigments) {
	for (index, theoretical) in original_layout_with_testimonies
		.layout
		.villagers
		.iter()
		.enumerate()
	{
		if theoretical.revealed
			&& let None = theoretical.inner.instance().testimony()
		{
			for (board_layout, ability_attempt, generated_testimonies) in theoretical_testimonies(
				&original_layout_with_testimonies.layout,
				VillagerIndex(index),
			) {
				let mut potential_assignments = original_layout_with_testimonies
					.satisfying_assignments
					.clone();

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

				yield (
					ability_attempt,
					LayoutWithTestimonyAssigments {
						layout: board_layout,
						satisfying_assignments: potential_assignments,
					},
				);
			}

			break;
		}
	}
}

gen fn theoretical_testimonies(
	board_config: &BoardLayout,
	testifier_index: VillagerIndex,
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

					for (expression, evil) in fortune_teller_expression(&index_combo) {
						let mut next_layout = board_config.clone();
						next_layout.description = format!(
							"{} - {} says {} or {} is{} evil",
							next_layout.description,
							testifier_index,
							index_combo[0],
							index_combo[1],
							if !evil { " NOT" } else { "" }
						);

						let instance_to_modify = next_layout.villagers[testifier_index.0]
							.inner
							.instance_mut();

						instance_to_modify.set_testimony(expression);

						let mut testimonies = vec![IndexTestimony::new(
							testifier_index.clone(),
							Testimony::FortuneTeller(FortuneTellerClaim::new(
								index_combo.as_slice().try_into().unwrap(),
								evil,
							)),
						)];

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
					if target_theoretical.actually_dead {
						continue;
					}

					let mut targets = BTreeSet::new();
					let target_index = VillagerIndex(target_index);
					targets.insert(target_index.clone());
					let ability_attempt = AbilityAttempt::new(testifier_index.clone(), targets);

					let mut next_layout = board_config.clone();

					let slayed;
					// TODO: The conditions for this may change. See https://discord.com/channels/1148903384968089640/1400926599628460052/1414305682290770012
					if testifier.inner.will_lie() {
						next_layout.description = format!(
							"{} - {} failed to slay {} due to lying",
							next_layout.description, testifier_index, target_index
						);
						slayed = false;
					} else if !target_theoretical.inner.true_identity().appears_evil() {
						next_layout.description = format!(
							"{} - {} failed to slay {} due to them not appearing evil",
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

					let truly_corrupt = target_theoretical.inner.corrupted();

					let says_corrupt = testifier.inner.will_lie() ^ truly_corrupt;
					let raw_testimony = Testimony::Corrupt(target_index.clone());

					let mut testimonies;
					let next_description;
					if says_corrupt {
						next_description = format!(
							"{} - {} {}correctly says: {} is corrupt",
							board_config.description,
							testifier_index,
							if truly_corrupt { "" } else { "IN" },
							target_index
						);
						testimonies = Vec::with_capacity(2);
					} else {
						next_description = format!(
							"{} - {} {}correctly says: {} is NOT corrupt",
							board_config.description,
							testifier_index,
							if truly_corrupt { "IN" } else { "" },
							target_index
						);
						testimonies = Vec::with_capacity(1);
					}

					testimonies.push(IndexTestimony::new(
						testifier_index.clone(),
						raw_testimony.clone(),
					));

					if !says_corrupt {
						let mut next_layout = board_config.clone();
						next_layout.description = next_description;
						let testifier_instance_to_modify = next_layout.villagers[testifier_index.0]
							.inner
							.instance_mut();
						testifier_instance_to_modify.set_testimony(Expression::Not(Box::new(
							Expression::Leaf(raw_testimony),
						)));
						yield (next_layout, ability_attempt, testimonies);
						continue;
					}

					// if they appear evil and we won't lie we say evil

					for (index, _) in board_config.villagers.iter().enumerate().filter(
						move |(index, villager)| {
							if *index == testifier_index.0 || *index == target_index.0 {
								false
							} else {
								testifier.inner.will_lie()
									^ villager.inner.true_identity().appears_evil()
							}
						},
					) {
						let evil_index = VillagerIndex(index);
						let next_description =
							format!("{}, {} is evil", next_description, evil_index);
						let mut next_layout = board_config.clone();
						next_layout.description = next_description;
						let testifier_instance_to_modify = next_layout.villagers[testifier_index.0]
							.inner
							.instance_mut();
						let evil_testimony = IndexTestimony {
							index: testifier_index.clone(),
							testimony: Testimony::Evil(evil_index),
						};
						testifier_instance_to_modify.set_testimony(Expression::And(
							Box::new(Expression::Leaf(raw_testimony.clone())),
							Box::new(Expression::Leaf(evil_testimony.testimony.clone())),
						));

						let mut testimonies: Vec<IndexTestimony> = testimonies.clone();
						testimonies.push(evil_testimony);
						yield (next_layout, ability_attempt.clone(), testimonies);
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

fn fortune_teller_expression(indexes: &Vec<VillagerIndex>) -> [(Expression<Testimony>, bool); 2] {
	[
		(
			Testimony::fortune_teller(
				indexes
					.as_slice()
					.try_into()
					.expect("Invalid number of indexes for a jester expression"),
				false,
			),
			false,
		),
		(
			Testimony::fortune_teller(indexes.as_slice().try_into().unwrap(), true),
			true,
		),
	]
}
