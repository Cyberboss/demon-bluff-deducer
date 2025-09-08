use std::{
	collections::{BTreeSet, HashSet},
	str::FromStr,
};

use demon_bluff_gameplay_engine::{
	affect::Affect,
	game_state::GameState,
	testimony::{AffectType, index_offset},
	villager::{
		ConfirmedVillager, GoodVillager, Minion, Outcast, Villager, VillagerArchetype,
		VillagerIndex, VillagerInstance,
	},
};
use itertools::Itertools;
use serde::{Deserialize, Serialize, de};

#[derive(Debug, PartialEq, Eq, Hash, Clone, PartialOrd, Ord, Serialize, Deserialize)]
pub struct TheoreticalVillager {
	pub inner: ConfirmedVillager,
	pub actually_dead: bool,
	pub was_corrupt: bool,
	pub revealed: bool,
	pub baked_from: Option<VillagerArchetype>,
	pub affection: Option<AffectType>,
}

impl TheoreticalVillager {
	pub fn new(value: ConfirmedVillager, dead: bool, revealed: bool) -> Self {
		let was_corrupt = value.corrupted();
		Self {
			actually_dead: dead,
			inner: value,
			was_corrupt,
			baked_from: None,
			affection: None,
			revealed,
		}
	}

	pub fn unknown_unrevealed_good(&self) -> bool {
		!self.revealed
			&& *self.inner.true_identity() == VillagerArchetype::GoodVillager(GoodVillager::Judge)
	}
}

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Serialize, Deserialize)]
pub struct BoardLayout {
	pub villagers: Vec<TheoreticalVillager>,
	pub evil_locations: BTreeSet<VillagerIndex>,
	pub description: String,
}

pub fn build_board_layouts(game_state: &GameState) -> HashSet<BoardLayout> {
	let mut remaining_initial_draw = Vec::with_capacity(game_state.deck().len());
	remaining_initial_draw.extend(game_state.deck().iter().cloned());
	remaining_initial_draw.sort();

	let mut disguisable_indicies = Vec::with_capacity(game_state.total_villagers());
	let mut remaining_evils =
		(game_state.draw_stats().minions() + game_state.draw_stats().demons()) as usize;
	let mut unrevealed_villagers = 0;
	let mut outcast_count = 0;

	game_state.iter_villagers(|index, villager| {
		match villager {
			Villager::Hidden(hidden_villager) => {
				if !hidden_villager.cant_kill() {
					disguisable_indicies.push(index);
				}

				unrevealed_villagers += 1;
			}
			Villager::Active(active_villager) => {
				if matches!(
					active_villager.instance().archetype(),
					VillagerArchetype::Outcast(_)
				) {
					outcast_count += 1;
				}

				if !active_villager.cant_kill() {
					disguisable_indicies.push(index);
				}
			}
			Villager::Confirmed(confirmed_villager) => {
				let true_identity = confirmed_villager.true_identity();
				if true_identity.is_evil() {
					remaining_evils -= 1;
				}

				if matches!(true_identity, VillagerArchetype::Outcast(_)) {
					outcast_count += 1;
				}

				remaining_initial_draw.retain(|deck_item| deck_item != true_identity);
			}
		}
		true
	});

	// there's probably redundancies in this loop but they will be deduplicated
	let mut layouts = HashSet::new();
	for disguise_index_combo in disguisable_indicies.iter().permutations(remaining_evils) {
		for evil_archetype_combo in remaining_initial_draw
			.iter()
			.filter(|archetype| archetype.is_evil())
			.permutations(remaining_evils)
		{
			assert_eq!(disguise_index_combo.len(), evil_archetype_combo.len());
			let mut first_desc = true;
			let mut desc = String::new();

			let confirmeds = game_state
				.villagers()
				.iter()
				.map(|villager| match villager {
					Villager::Active(active_villager) => {
						(Some((active_villager.instance(), None)), false)
					}
					Villager::Hidden(hidden_villager) => (None, hidden_villager.dead()),
					Villager::Confirmed(confirmed_villager) => (
						Some((confirmed_villager.instance(), Some(confirmed_villager))),
						true,
					),
				})
				.enumerate()
				.map(|(index, (instance_and_confirmed, dead))| {
					let index = VillagerIndex(index);

					if let Some(disguise_index) = disguise_index_combo
						.iter()
						.position(|iterated_index| iterated_index == &&index)
					{
						let evil_archetype = evil_archetype_combo[disguise_index];
						let disguised_archetype = (*evil_archetype).clone();

						let mut unknown_hidden = false;
						let instance = match instance_and_confirmed {
							Some((instance, _)) => instance.clone(),
							None => {
								// for our purposes, the instance doesn't matter here
								unknown_hidden = true;
								VillagerInstance::new(
									VillagerArchetype::GoodVillager(GoodVillager::Confessor),
									None,
								)
							}
						};

						if !first_desc {
							desc = format!("{}, ", desc);
						} else {
							first_desc = false;
						}
						desc = format!(
							"{}{}: {} (actually a {})",
							desc,
							index,
							if unknown_hidden {
								String::from_str("Unknown").unwrap()
							} else {
								format!("{}", instance.archetype())
							},
							disguised_archetype
						);

						TheoreticalVillager::new(
							ConfirmedVillager::new(
								instance.clone(),
								Some(disguised_archetype),
								false,
							),
							false,
							!unknown_hidden,
						)
					} else {
						let mut unknown_hidden = false;
						let theoretical = if let Some((instance, confirmed)) =
							instance_and_confirmed
						{
							if let Some(confirmed) = confirmed {
								TheoreticalVillager::new(confirmed.clone(), dead, !unknown_hidden)
							} else {
								let corrupt = instance.archetype().starts_corrupted();
								TheoreticalVillager::new(
									ConfirmedVillager::new(instance.clone(), None, corrupt),
									dead,
									!unknown_hidden,
								)
							}
						} else {
							unknown_hidden = true;
							let good_archetype =
								VillagerArchetype::GoodVillager(GoodVillager::Judge);
							let corrupt = good_archetype.starts_corrupted();
							TheoreticalVillager::new(
								ConfirmedVillager::new(
									VillagerInstance::new(good_archetype.clone(), None),
									None,
									corrupt,
								),
								dead,
								!unknown_hidden,
							)
						};

						if !first_desc {
							desc = format!("{}, ", desc);
						} else {
							first_desc = false;
						}
						desc = format!(
							"{}{}: {}",
							desc,
							index,
							if unknown_hidden {
								String::from_str("Unknown").unwrap()
							} else {
								format!("{}", theoretical.inner.true_identity())
							},
						);

						theoretical
					}
				})
				.collect();

			let evil_locations = disguise_index_combo
				.iter()
				.map(|index| (*index).clone())
				.collect();

			// TODO: Test pass order once deck builder mode releases
			let adjacency_affected_theoreticals =
				with_adjacent_affects(game_state, confirmeds, evil_locations, desc);
			let counsellor_affected_theoreticals = adjacency_affected_theoreticals
				.into_iter()
				.flat_map(with_counsellors);
			// TODO: Shaman (Cloner) pass
			// TODO: Doppleganger pass
			let doppleganger_affected_theoreticals = counsellor_affected_theoreticals
				.into_iter()
				.flat_map(with_dopplegangers);
			let plague_doctor_affected_theoreticals = doppleganger_affected_theoreticals
				.into_iter()
				.flat_map(|layout| with_plague_doctors(game_state, layout));
			// TODO: Alchemist pass
			// TODO: Drunk pass (alchemist cannot cure)
			// TODO: Baker pass

			layouts.extend(plague_doctor_affected_theoreticals);
		}
	}

	layouts
}

fn with_adjacent_affects(
	game_state: &GameState,
	mut initial_theoreticals: Vec<TheoreticalVillager>,
	evil_locations: BTreeSet<VillagerIndex>,
	base_desc: String,
) -> Vec<BoardLayout> {
	let mut any_affects_applied = false;

	let mut with_affects = Vec::new();

	let mut affecting_indicies = Vec::new();

	for (index, theoretical) in initial_theoreticals.iter_mut().enumerate() {
		let index = VillagerIndex(index);
		if let Some(affect) = theoretical
			.inner
			.true_identity()
			.affect(game_state.total_villagers(), Some(index.clone()))
		{
			match affect {
				Affect::Corrupt(_) | Affect::Puppet(_) => {
					any_affects_applied = true;
					affecting_indicies.push(index);
				}
				Affect::DupeVillager
				| Affect::Night(_)
				| Affect::Outcast(_)
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
		for distribution_permutation in generate_boolean_permutations(affect_permutation.len()) {
			let mut mutated_confirmeds = initial_theoreticals.clone();
			let mut mutated_desc = base_desc.clone();
			for i in 0..affect_permutation.len() {
				let to_the_left = distribution_permutation[i];
				let affecting_index = affect_permutation[i];
				let affector = &initial_theoreticals[affecting_index.0];
				let affected_index = index_offset(
					affecting_index,
					game_state.total_villagers(),
					1,
					to_the_left,
				);
				let affected_villager = &mut mutated_confirmeds[affected_index.0];
				match affector
					.inner
					.true_identity()
					.affect(game_state.total_villagers(), Some(affecting_index.clone()))
					.expect("Affect should be here!")
				{
					Affect::Corrupt(_) => {
						// plague doctor handled in another pass
						if *affector.inner.true_identity()
							!= VillagerArchetype::Outcast(Outcast::PlagueDoctor)
							&& affected_villager.inner.true_identity().can_be_corrupted()
							&& !affected_villager.inner.corrupted()
						{
							mutated_desc =
								format!("{} - {} was corrupted", mutated_desc, affected_index);
							affected_villager.inner = ConfirmedVillager::new(
								affected_villager.inner.instance().clone(),
								affected_villager.inner.hidden_identity().clone(),
								true,
							);
							if affector.inner.true_identity().is_evil() {
								affected_villager.affection = Some(AffectType::CorruptedByEvil);
							}

							affected_villager.was_corrupt = true;
						}
					}
					Affect::Puppet(_) => {
						if affected_villager.inner.true_identity().can_be_converted() {
							mutated_desc =
								format!("{} - {} was puppeted", mutated_desc, affected_index);
							affected_villager.inner = ConfirmedVillager::new(
								affected_villager.inner.instance().clone(),
								Some(VillagerArchetype::Minion(Minion::Puppet)),
								false,
							);
							affected_villager.affection = Some(AffectType::Puppeted);
						}
					}
					Affect::Outcast(_) => {
						// handled in another pass
					}
					Affect::DupeVillager => {
						// handled in another pass
					}
					Affect::FakeOutcast | Affect::BlockLastNReveals(_) | Affect::Night(_) => {
						panic!("This isn't a villager affect!")
					}
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
			villagers: initial_theoreticals,
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

gen fn with_counsellors(layout: BoardLayout) -> BoardLayout {
	let mut affectable_indicies = Vec::with_capacity(layout.villagers.len() - 1);
	for (index, _) in layout.villagers.iter().enumerate().filter(|(_, villager)| {
		*villager.inner.true_identity() == VillagerArchetype::Minion(Minion::Counsellor)
	}) {
		let villager_index = VillagerIndex(index);
		let consellor_affectable_indicies = [
			index_offset(&villager_index, layout.villagers.len(), 1, true),
			index_offset(&villager_index, layout.villagers.len(), 1, false),
		];
		affectable_indicies.push((villager_index, consellor_affectable_indicies));
	}

	if affectable_indicies.len() == 0 {
		yield layout;
		return;
	}

	// true is a left selection
	for permutation in generate_boolean_permutations(affectable_indicies.len()) {
		for (affectable_indicies_index, left_pick) in permutation.into_iter().enumerate() {
			let (source_consellor, affectable_indicies) =
				&affectable_indicies[affectable_indicies_index];
			let target_index = &affectable_indicies[if left_pick { 0 } else { 1 }];

			if matches!(
				layout.villagers[target_index.0].inner.true_identity(),
				VillagerArchetype::Outcast(_)
			) {
				let mut next_layout = layout.clone();
				next_layout.description = format!(
					"{} - {} was converted to an outcast by {}",
					layout.description, target_index, source_consellor
				);
				next_layout.villagers[target_index.0].affection = Some(AffectType::Outcasted);
				yield next_layout;
			}
		}
	}
}

gen fn with_dopplegangers(layout: BoardLayout) -> BoardLayout {
	let num_gangers = layout
		.villagers
		.iter()
		.filter(|villager| {
			*villager.inner.true_identity() == VillagerArchetype::Outcast(Outcast::Doppelganger)
		})
		.count();

	if num_gangers == 0 {
		yield layout;
		return;
	}

	todo!("Doppleganger cloning");
}

gen fn with_plague_doctors(game_state: &GameState, layout: BoardLayout) -> BoardLayout {
	if !game_state.role_in_play(VillagerArchetype::Outcast(Outcast::PlagueDoctor)) {
		yield layout;
		return;
	}

	let affectable_indicies: Vec<usize> = layout
		.villagers
		.iter()
		.enumerate()
		.filter_map(|(index, villager)| {
			if !villager.inner.corrupted() && villager.inner.true_identity().can_be_corrupted() {
				Some(index)
			} else {
				None
			}
		})
		.collect();

	for plague_doctor_layout in with_real_plague_doctor_locations(layout) {
		let mut any_affecteable_indicies = false;

		// check there actually is a PD in the layout
		if plague_doctor_layout.villagers.iter().any(|villager| {
			*villager.inner.true_identity() == VillagerArchetype::Outcast(Outcast::PlagueDoctor)
		}) {
			for index in affectable_indicies.clone().into_iter() {
				any_affecteable_indicies = true;
				let mut next_layout = plague_doctor_layout.clone();
				let mutated_theoretical = &mut next_layout.villagers[index];
				mutated_theoretical.was_corrupt = true;
				mutated_theoretical.inner.set_corrupted(true);
				next_layout.description = format!(
					"{} - {} was corrupted by the PD",
					next_layout.description,
					VillagerIndex(index),
				);

				yield next_layout;
			}
		}

		if !any_affecteable_indicies {
			yield plague_doctor_layout;
		}
	}
}

gen fn with_real_plague_doctor_locations(layout: BoardLayout) -> BoardLayout {
	if layout.villagers.iter().all(|villager| {
		*villager.inner.true_identity() != VillagerArchetype::Outcast(Outcast::PlagueDoctor)
	}) {
		for (index, theoretical) in layout.villagers.iter().enumerate() {
			if !theoretical.inner.true_identity().is_evil()
				&& !theoretical.inner.corrupted()
				&& !theoretical.revealed
			{
				let mut next_layout = layout.clone();
				next_layout.description = format!(
					"{} - {} is PD",
					next_layout.description,
					VillagerIndex(index)
				);
				next_layout.villagers[index].inner = ConfirmedVillager::new(
					VillagerInstance::new(VillagerArchetype::Outcast(Outcast::PlagueDoctor), None),
					None,
					false,
				);
			}
		}
	}

	// just in case the PD is fake
	yield layout;
}
