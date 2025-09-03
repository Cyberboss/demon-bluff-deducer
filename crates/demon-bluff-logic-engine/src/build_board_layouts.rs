use std::{
	arch::breakpoint,
	collections::{BTreeSet, HashSet},
};

use demon_bluff_gameplay_engine::{
	affect::Affect,
	game_state::GameState,
	testimony::index_offset,
	villager::{
		ConfirmedVillager, GoodVillager, Minion, Villager, VillagerArchetype, VillagerIndex,
		VillagerInstance,
	},
};
use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct BoardLayout {
	pub villagers: Vec<ConfirmedVillager>,
	pub evil_locations: BTreeSet<VillagerIndex>,
	pub description: String,
}

pub fn build_board_layouts(game_state: &GameState) -> HashSet<BoardLayout> {
	let mut remaining_initial_draw = HashSet::with_capacity(game_state.deck().len());
	remaining_initial_draw.extend(game_state.deck().iter().cloned());

	let mut disguisable_indicies = Vec::with_capacity(game_state.total_villagers());
	let mut remaining_evils =
		(game_state.draw_stats().minions() + game_state.draw_stats().demons()) as usize;
	let mut unrevealed_villagers = 0;
	let mut outcast_count = 0;

	game_state.iter_villagers(|index, villager| match villager {
		Villager::Hidden(_) => {
			disguisable_indicies.push(index);
			unrevealed_villagers += 1;
		}
		Villager::Active(active_villager) => {
			if matches!(
				active_villager.instance().archetype(),
				VillagerArchetype::Outcast(_)
			) {
				outcast_count += 1;
			}
			disguisable_indicies.push(index);
		}
		Villager::Confirmed(confirmed_villager) => {
			let true_identity = confirmed_villager.true_identity();
			if true_identity.is_evil() {
				remaining_evils -= 1;
			}

			if matches!(true_identity, VillagerArchetype::Outcast(_)) {
				outcast_count += 1;
			}

			remaining_initial_draw.remove(true_identity);
		}
	});

	let extra_outcasts = if outcast_count > game_state.draw_stats().outcasts() {
		outcast_count - game_state.draw_stats().outcasts()
	} else {
		0
	};

	// there's probably redundancies in this loop but they will be deduplicated
	// TODO: Consider doppleganger
	// TODO: Consider baker
	let mut layouts = HashSet::new();
	for disguise_index_combo in disguisable_indicies.iter().permutations(remaining_evils) {
		for evil_archetype_combo in remaining_initial_draw
			.iter()
			.filter(|archetype| archetype.is_evil())
			.permutations(remaining_evils)
		{
			for good_archetype_combo in remaining_initial_draw
				.iter()
				.filter(|archetype| !archetype.is_evil())
				.combinations(unrevealed_villagers - remaining_evils)
			{
				for disguise_index in &disguise_index_combo {
					for evil_archetype in &evil_archetype_combo {
						for good_archetype in &good_archetype_combo {
							let mut first_desc = true;
							let mut desc = String::new();

							let confirmeds = game_state
								.villagers()
								.iter()
								.map(|villager| match villager {
									Villager::Active(active_villager) => {
										Some((active_villager.instance(), None))
									}
									Villager::Hidden(_) => None,
									Villager::Confirmed(confirmed_villager) => Some((
										confirmed_villager.instance(),
										Some(confirmed_villager),
									)),
								})
								.enumerate()
								.map(|(index, instance_and_confirmed)| {
									let index = VillagerIndex(index);

									if index == **disguise_index {
										let disguised_archetype = (*evil_archetype).clone();
										if !first_desc {
											desc = format!("{} - ", desc);
										} else {
											first_desc = false;
										}

										desc = format!(
											"{}{} actually a {}",
											desc, index, disguised_archetype
										);

										ConfirmedVillager::new(
											(match instance_and_confirmed {
												Some((instance, _)) => instance.clone(),
												None => {
													// for our purposes, the instance doesn't matter here
													VillagerInstance::new(
														VillagerArchetype::GoodVillager(
															GoodVillager::Confessor,
														),
														None,
													)
												}
											})
											.clone(),
											Some(disguised_archetype),
											false,
										)
									} else if let Some((instance, confirmed)) =
										instance_and_confirmed
									{
										if let Some(confirmed) = confirmed {
											confirmed.clone()
										} else {
											let corrupt = instance.archetype().starts_corrupted();
											ConfirmedVillager::new(instance.clone(), None, corrupt)
										}
									} else {
										let corrupt = good_archetype.starts_corrupted();
										ConfirmedVillager::new(
											VillagerInstance::new((*good_archetype).clone(), None),
											None,
											corrupt,
										)
									}
								})
								.collect();

							let evil_locations = disguise_index_combo
								.iter()
								.map(|index| (*index).clone())
								.collect();
							let affected_confirmeds = with_affects(
								game_state,
								confirmeds,
								extra_outcasts,
								evil_locations,
								desc,
							);

							layouts.extend(affected_confirmeds);
						}
					}
				}
			}
		}
	}

	layouts
}

fn with_affects(
	game_state: &GameState,
	mut initial_confirmeds: Vec<ConfirmedVillager>,
	extra_outcasts: u8,
	evil_locations: BTreeSet<VillagerIndex>,
	base_desc: String,
) -> Vec<BoardLayout> {
	let mut any_affects_applied = false;

	let mut with_affects = Vec::new();

	let mut affecting_indicies = Vec::new();

	for (index, confirmed) in initial_confirmeds.iter_mut().enumerate() {
		let index = VillagerIndex(index);
		if let Some(affect) = confirmed
			.true_identity()
			.affect(game_state.total_villagers(), Some(index.clone()))
		{
			match affect {
				Affect::Corrupt(_) | Affect::Puppet(_) | Affect::Outcast(_) => {
					any_affects_applied = true;
					affecting_indicies.push(index);
				}
				Affect::DupeVillager
				| Affect::Night(_)
				| Affect::FakeOutcast
				| Affect::BlockLastNReveals(_) => {}
			}
		}
	}

	// because affects can cancel other affects, try in every possible order
	for affect_permutation in affecting_indicies
		.iter()
		.permutations(affecting_indicies.len())
	{
		// TODO: currently all VillagerAffects are adjacency affects. Change this if that every changes
		// TODO: I eat my words already, this won't work for PlagueDoctors
		for distribution_permutation in generate_boolean_permutations(affect_permutation.len()) {
			let mut mutated_confirmeds = initial_confirmeds.clone();
			let mut mutated_desc = base_desc.clone();
			for i in 0..affect_permutation.len() {
				let to_the_left = distribution_permutation[i];
				let affecting_index = affect_permutation[i];
				let confirmed = &initial_confirmeds[affecting_index.0];
				let affected_index = index_offset(
					affecting_index,
					game_state.total_villagers(),
					1,
					to_the_left,
				);
				let affected_villager = &mut mutated_confirmeds[affected_index.0];
				match confirmed
					.true_identity()
					.affect(game_state.total_villagers(), Some(affecting_index.clone()))
					.expect("Affect should be here!")
				{
					Affect::Corrupt(_) => {
						if affected_villager.true_identity().can_be_corrupted()
							&& !affected_villager.corrupted()
						{
							mutated_desc =
								format!("{} - {} was corrupted", mutated_desc, affected_index);
							mutated_confirmeds[affected_index.0] = ConfirmedVillager::new(
								affected_villager.instance().clone(),
								affected_villager.hidden_identity().clone(),
								true,
							);
						}
					}
					Affect::Puppet(_) => {
						if affected_villager.true_identity().can_be_converted() {
							mutated_desc =
								format!("{} - {} was puppeted", mutated_desc, affected_index);
							mutated_confirmeds[affected_index.0] = ConfirmedVillager::new(
								affected_villager.instance().clone(),
								Some(VillagerArchetype::Minion(Minion::Puppet)),
								false,
							);
						}
					}
					Affect::Outcast(_) => {
						// TODO: this outcast conversion process sucks, make it better
						if extra_outcasts == 0
							&& affected_villager.true_identity().can_be_converted()
						{
							mutated_desc = format!(
								"{} - {} was converted to an outcast",
								mutated_desc, affected_index
							);
							mutated_confirmeds[affected_index.0] = ConfirmedVillager::new(
								VillagerInstance::new(
									game_state
										.deck()
										.iter()
										.filter(|archetype| {
											matches!(archetype, VillagerArchetype::Outcast(_))
										})
										.next()
										.cloned()
										.expect("There wasn't at least one outcast?"),
									None,
								),
								Some(VillagerArchetype::Minion(Minion::Puppet)),
								false,
							);
						}
					}
					Affect::DupeVillager
					| Affect::FakeOutcast
					| Affect::BlockLastNReveals(_)
					| Affect::Night(_) => panic!("This isn't a villager affect!"),
				}
			}

			with_affects.push(BoardLayout {
				villagers: mutated_confirmeds,
				description: mutated_desc,
				evil_locations: evil_locations.clone(),
			});
		}
	}

	if !any_affects_applied {
		with_affects.push(BoardLayout {
			villagers: initial_confirmeds,
			description: base_desc,
			evil_locations: evil_locations,
		});
	}

	with_affects
}

fn generate_boolean_permutations(n: usize) -> Vec<Vec<bool>> {
	let mut permutations = Vec::new();
	let num_combinations = 1 << n; // 2^n combinations

	for i in 0..num_combinations {
		let mut current_permutation = Vec::with_capacity(n);
		for j in 0..n {
			// Check the j-th bit of i
			if (i >> j) & 1 == 1 {
				current_permutation.push(true);
			} else {
				current_permutation.push(false);
			}
		}
		permutations.push(current_permutation);
	}
	permutations
}
