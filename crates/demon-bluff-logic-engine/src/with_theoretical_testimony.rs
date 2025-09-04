use std::{
	arch::breakpoint,
	collections::{BTreeSet, HashMap, HashSet},
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
) -> HashMap<BoardLayout, AbilityAttempt> {
	let mut results = HashMap::new();
	for inital_board_config in board_configs {
		for (index, theoretical) in inital_board_config.villagers.iter().enumerate() {
			if let None = theoretical.inner.instance().testimony() {
				for (layout, ability_attempt) in
					theoretical_testimonies(game_state, &inital_board_config, VillagerIndex(index))
				{
					results.insert(layout, ability_attempt);
				}
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
				for (index, theoretical) in theoreticals.iter().enumerate() {
					let index = VillagerIndex(index);

					let mut targets = HashSet::with_capacity(1);
					targets.insert(index.clone());
					if index.0 == 2 && testifier_index.0 == 4 {
						breakpoint();
					}

					let mut base_expr = Expression::Leaf(Testimony::Lying(index.clone()));
					let wont_lie = !theoretical.inner.will_lie();
					if wont_lie {
						base_expr = Expression::Not(Box::new(base_expr));
					}

					let mut next_layout = board_config.clone();
					next_layout.villagers[testifier_index.0]
						.inner
						.instance_mut()
						.set_testimony(lie_if_required(base_expr, testifier));

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

					if index.0 == 0 || index.0 == 4 {
						next_layout.of_interest = true;
					}

					yield (
						next_layout,
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

fn lie_if_required(
	mut testimony: Expression<Testimony>,
	theoretical: &TheoreticalVillager,
) -> Expression<Testimony> {
	if theoretical.inner.will_lie() {
		breakpoint();
		testimony = Expression::Not(Box::new(testimony));
	}

	testimony
}
