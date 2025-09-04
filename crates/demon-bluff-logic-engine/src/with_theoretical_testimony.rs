use std::{
	arch::breakpoint,
	collections::{BTreeSet, HashMap, HashSet, VecDeque},
	thread::yield_now,
};

use demon_bluff_gameplay_engine::{
	Expression,
	game_state::{self, GameState},
	testimony::Testimony,
	villager::{self, GoodVillager, Outcast, VillagerArchetype, VillagerIndex},
};

use crate::{
	build_board_layouts::{BoardLayout, TheoreticalVillager},
	player_action::AbilityAttempt,
};

pub fn with_theoretical_testimony(
	game_state: &GameState,
	board_configs: impl IntoIterator<Item = BoardLayout>,
) -> HashMap<AbilityAttempt, Vec<BoardLayout>> {
	let board_configs: Vec<BoardLayout> = board_configs.into_iter().collect();
	let mut iterators = Vec::new();
	for initial_board_config in &board_configs {
		iterators.push(iter_board_villagers(game_state, initial_board_config));
	}

	let mut results = HashMap::with_capacity(iterators[0].len());
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

	results
}

fn iter_board_villagers(
	game_state: &GameState,
	inital_board_config: &BoardLayout,
) -> VecDeque<(BoardLayout, AbilityAttempt)> {
	let mut results = VecDeque::new();
	for (index, theoretical) in inital_board_config.villagers.iter().enumerate() {
		if let None = theoretical.inner.instance().testimony() {
			for tuple in
				theoretical_testimonies(game_state, &inital_board_config, VillagerIndex(index))
			{
				results.push_back(tuple);
			}
		}
	}

	results
}

gen fn theoretical_testimonies(
	game_state: &GameState,
	board_config: &BoardLayout,
	testifier_index: VillagerIndex,
) -> (BoardLayout, AbilityAttempt) {
	let theoreticals = &board_config.villagers;
	let testifier = &theoreticals[testifier_index.0];
	let archetype = testifier.inner.instance().archetype();
	match archetype {
		VillagerArchetype::GoodVillager(good_villager) => match good_villager {
			GoodVillager::Alchemist => todo!("Alchemist testimony generation"),
			GoodVillager::Architect => todo!("Architect testimony generation"),
			GoodVillager::Baker => todo!("Baker testimony generation"),
			GoodVillager::Bard => todo!("Bard testimony generation"),
			GoodVillager::Bishop => todo!("Bishop testimony generation"),
			GoodVillager::Confessor => todo!("Confessor testimony generation"),
			GoodVillager::Dreamer => todo!("Dreamer testimony generation"),
			GoodVillager::Druid => todo!("Druid testimony generation"),
			GoodVillager::Empress => todo!("Empress testimony generation"),
			GoodVillager::Enlightened => todo!("Enlightened testimony generation"),
			GoodVillager::FortuneTeller => todo!("FortuneTeller testimony generation"),
			GoodVillager::Gemcrafter => todo!("Gemcrafter testimony generation"),
			GoodVillager::Hunter => todo!("Hunter testimony generation"),
			GoodVillager::Jester => todo!("Jester testimony generation"),
			GoodVillager::Judge => {
				for (index, _) in theoreticals.iter().enumerate() {
					let index = VillagerIndex(index);

					let mut targets = HashSet::with_capacity(1);
					targets.insert(index.clone());
					if index.0 == 2 && testifier_index.0 == 4 {
						breakpoint();
					}

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

					let mut next_layout2 = next_layout.clone();
					yield (
						next_layout,
						AbilityAttempt::new(testifier_index.clone(), targets.clone()),
					);

					let negative_testimony = Expression::Not(Box::new(
						next_layout2.villagers[testifier_index.0]
							.inner
							.instance()
							.testimony()
							.as_ref()
							.unwrap()
							.clone(),
					));
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
					yield (
						next_layout2,
						AbilityAttempt::new(testifier_index.clone(), targets),
					);
				}
			}
			GoodVillager::Knight => todo!("Knight testimony generation"),
			GoodVillager::Knitter => todo!("Knitter testimony generation"),
			GoodVillager::Lover => todo!("Lover testimony generation"),
			GoodVillager::Medium => todo!("Medium testimony generation"),
			GoodVillager::Oracle => todo!("Oracle testimony generation"),
			GoodVillager::Poet => todo!("FUCKING POET TESTIMONY GENERATION!!!"),
			GoodVillager::Scout => todo!("Scout testimony generation"),
			GoodVillager::Slayer => todo!("Slayer testimony generation"),
			GoodVillager::Witness => todo!("Witness testimony generation"),
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
