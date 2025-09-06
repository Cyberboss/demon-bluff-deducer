use std::{
	arch::breakpoint,
	collections::{HashMap, HashSet, VecDeque},
};

use demon_bluff_gameplay_engine::{
	Expression,
	testimony::Testimony,
	villager::{GoodVillager, Outcast, VillagerArchetype, VillagerIndex},
};
use itertools::Itertools;

use crate::{build_board_layouts::BoardLayout, player_action::AbilityAttempt};

pub fn with_theoretical_testimony(
	board_configs: impl IntoIterator<Item = BoardLayout>,
) -> HashMap<AbilityAttempt, Vec<BoardLayout>> {
	let board_configs: Vec<BoardLayout> = board_configs.into_iter().collect();
	let mut iterators = Vec::new();
	for initial_board_config in &board_configs {
		iterators.push(iter_board_villagers_once(initial_board_config));
	}

	let mut results: HashMap<AbilityAttempt, Vec<BoardLayout>> =
		HashMap::with_capacity(iterators[0].len());
	loop {
		let mut group: Vec<(BoardLayout, AbilityAttempt)> = Vec::new();
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
			group.into_iter().map(|(layout, _)| layout).collect(),
		);
	}

	// recursively expand every solution until all testimonies are acquired
	let mut any_potential_testimonies_remaining = false;
	'outer: for potential_layout in results
		.iter()
		.flat_map(|(_, potential_layouts)| potential_layouts.iter())
	{
		for theoretical in &potential_layout.villagers {
			if theoretical.inner.instance().testimony().is_none() {
				any_potential_testimonies_remaining = true;
				break 'outer;
			}
		}
	}

	if any_potential_testimonies_remaining {
		let mut expanded_results = HashMap::new();
		for (ability_attempt, new_layouts) in results {
			let expanded_layouts = with_theoretical_testimony(new_layouts);
			let total_expanded_layouts = expanded_layouts
				.into_iter()
				.flat_map(|(_, expanded_layouts)| expanded_layouts.into_iter())
				.collect();

			expanded_results.insert(ability_attempt, total_expanded_layouts);
		}

		expanded_results
	} else {
		results
	}
}

fn iter_board_villagers_once(
	inital_board_config: &BoardLayout,
) -> VecDeque<(BoardLayout, AbilityAttempt)> {
	let mut results = VecDeque::new();
	for (index, theoretical) in inital_board_config.villagers.iter().enumerate() {
		if theoretical.revealed
			&& let None = theoretical.inner.instance().testimony()
		{
			for tuple in theoretical_testimonies(&inital_board_config, VillagerIndex(index)) {
				results.push_back(tuple);
			}

			break;
		}
	}

	results
}

gen fn theoretical_testimonies(
	board_config: &BoardLayout,
	testifier_index: VillagerIndex,
) -> (BoardLayout, AbilityAttempt) {
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
			GoodVillager::FortuneTeller => todo!("FortuneTeller testimony generation"),
			GoodVillager::Jester => {
				for index_combo in theoreticals
					.iter()
					.enumerate()
					.map(|(index, _)| VillagerIndex(index))
					.combinations(3)
				{
					for expression in jester_expression(&index_combo) {
						let mut targets = HashSet::with_capacity(3);
						targets.extend(index_combo.iter().cloned());
						let mut next_layout = board_config.clone();
						next_layout.description = format!(
							"{} - {} says {}",
							next_layout.description, testifier_index, expression
						);

						let instance_to_modify = next_layout.villagers[testifier_index.0]
							.inner
							.instance_mut();

						instance_to_modify.set_testimony(expression);

						if next_layout.villagers[0]
							.inner
							.instance()
							.testimony()
							.is_none() || next_layout.villagers[1]
							.inner
							.instance()
							.testimony()
							.is_none() || next_layout.villagers[3]
							.inner
							.instance()
							.testimony()
							.is_none() || next_layout.villagers[4]
							.inner
							.instance()
							.testimony()
							.is_none()
						{
							breakpoint();
						}

						yield (
							next_layout,
							AbilityAttempt::new(testifier_index.clone(), targets),
						);
					}
				}
			}
			GoodVillager::Judge => {
				for (index, _) in theoreticals.iter().enumerate() {
					let index = VillagerIndex(index);

					let mut targets = HashSet::with_capacity(1);
					targets.insert(index.clone());

					let base_expr = Expression::Leaf(Testimony::Lying(index.clone()));

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

					if next_layout.villagers[0]
						.inner
						.instance()
						.testimony()
						.is_none() || next_layout.villagers[1]
						.inner
						.instance()
						.testimony()
						.is_none() || next_layout.villagers[3]
						.inner
						.instance()
						.testimony()
						.is_none() || next_layout.villagers[4]
						.inner
						.instance()
						.testimony()
						.is_none()
					{
						breakpoint();
					}

					yield (
						next_layout,
						AbilityAttempt::new(testifier_index.clone(), targets.clone()),
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

					if next_layout2.villagers[0]
						.inner
						.instance()
						.testimony()
						.is_none() || next_layout2.villagers[1]
						.inner
						.instance()
						.testimony()
						.is_none() || next_layout2.villagers[3]
						.inner
						.instance()
						.testimony()
						.is_none() || next_layout2.villagers[4]
						.inner
						.instance()
						.testimony()
						.is_none()
					{
						breakpoint();
					}

					yield (
						next_layout2,
						AbilityAttempt::new(testifier_index.clone(), targets),
					);
				}
			}
			GoodVillager::Slayer => todo!("Slayer testimony generation"),
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
			Outcast::PlagueDoctor => todo!(),
		},
		VillagerArchetype::Minion(_) | VillagerArchetype::Demon(_) => {
			panic!("A {} should not have a testimony!", archetype)
		}
	}
}

fn jester_expression(indexes: &Vec<VillagerIndex>) -> [Expression<Testimony>; 4] {
	assert_eq!(3, indexes.len());
	[
		Expression::And(
			Box::new(Expression::Not(Box::new(Expression::Leaf(
				Testimony::Evil(indexes[0].clone()),
			)))),
			Box::new(Expression::And(
				Box::new(Expression::Not(Box::new(Expression::Leaf(
					Testimony::Evil(indexes[1].clone()),
				)))),
				Box::new(Expression::Not(Box::new(Expression::Leaf(
					Testimony::Evil(indexes[2].clone()),
				)))),
			)),
		),
		Expression::Or(
			Box::new(Expression::And(
				Box::new(Expression::Leaf(Testimony::Evil(indexes[0].clone()))),
				Box::new(Expression::And(
					Box::new(Expression::Not(Box::new(Expression::Leaf(
						Testimony::Evil(indexes[1].clone()),
					)))),
					Box::new(Expression::Not(Box::new(Expression::Leaf(
						Testimony::Evil(indexes[2].clone()),
					)))),
				)),
			)),
			Box::new(Expression::Or(
				Box::new(Expression::And(
					Box::new(Expression::Not(Box::new(Expression::Leaf(
						Testimony::Evil(indexes[0].clone()),
					)))),
					Box::new(Expression::And(
						Box::new(Expression::Leaf(Testimony::Evil(indexes[1].clone()))),
						Box::new(Expression::Not(Box::new(Expression::Leaf(
							Testimony::Evil(indexes[2].clone()),
						)))),
					)),
				)),
				Box::new(Expression::And(
					Box::new(Expression::Not(Box::new(Expression::Leaf(
						Testimony::Evil(indexes[0].clone()),
					)))),
					Box::new(Expression::And(
						Box::new(Expression::Not(Box::new(Expression::Leaf(
							Testimony::Evil(indexes[1].clone()),
						)))),
						Box::new(Expression::Leaf(Testimony::Evil(indexes[2].clone()))),
					)),
				)),
			)),
		),
		Expression::Or(
			Box::new(Expression::And(
				Box::new(Expression::Not(Box::new(Expression::Leaf(
					Testimony::Evil(indexes[0].clone()),
				)))),
				Box::new(Expression::And(
					Box::new(Expression::Leaf(Testimony::Evil(indexes[1].clone()))),
					Box::new(Expression::Leaf(Testimony::Evil(indexes[2].clone()))),
				)),
			)),
			Box::new(Expression::Or(
				Box::new(Expression::And(
					Box::new(Expression::Leaf(Testimony::Evil(indexes[0].clone()))),
					Box::new(Expression::And(
						Box::new(Expression::Not(Box::new(Expression::Leaf(
							Testimony::Evil(indexes[1].clone()),
						)))),
						Box::new(Expression::Leaf(Testimony::Evil(indexes[2].clone()))),
					)),
				)),
				Box::new(Expression::And(
					Box::new(Expression::Leaf(Testimony::Evil(indexes[0].clone()))),
					Box::new(Expression::And(
						Box::new(Expression::Leaf(Testimony::Evil(indexes[1].clone()))),
						Box::new(Expression::Not(Box::new(Expression::Leaf(
							Testimony::Evil(indexes[2].clone()),
						)))),
					)),
				)),
			)),
		),
		Expression::And(
			Box::new(Expression::Leaf(Testimony::Evil(indexes[0].clone()))),
			Box::new(Expression::And(
				Box::new(Expression::Leaf(Testimony::Evil(indexes[1].clone()))),
				Box::new(Expression::Leaf(Testimony::Evil(indexes[2].clone()))),
			)),
		),
	]
}
