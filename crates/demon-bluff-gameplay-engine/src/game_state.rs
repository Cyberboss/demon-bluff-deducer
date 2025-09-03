use std::mem::replace;

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{
	Expression,
	testimony::Testimony,
	villager::{
		ActiveVillager, ConfirmedVillager, Demon, ExecutionResult, GoodVillager, HiddenVillager,
		Minion, Outcast, Villager, VillagerArchetype, VillagerIndex, VillagerInstance,
	},
};

pub const DAYS_BEFORE_NIGHT: u8 = 4;

pub enum DayCycle {
	Day1,
	Day2,
	Day3,
	Day4,
	Night,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DrawStats {
	villagers: u8,
	outcasts: u8,
	minions: u8,
	demons: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
	// TODO: Alignment of cards may affect Architect claim, double check
	next_day: Option<u8>,
	draw_stats: DrawStats,
	deck: Vec<VillagerArchetype>,
	villagers: Vec<Villager>,
	reveal_order: Vec<VillagerIndex>,
	hitpoints: u8,
	total_evils: u8,
}

#[derive(Debug)]
pub struct RevealResult {
	index: VillagerIndex,
	instance: Option<VillagerInstance>,
}

#[derive(Debug)]
pub struct KillAttempt {
	target: VillagerIndex,
	result: Option<KillResult>,
}

#[derive(Debug)]
pub enum KillResult {
	Unrevealed(UnrevealedKillData),
	Revealed(KillData),
}

#[derive(Debug)]
pub struct UnrevealedKillData {
	identity: VillagerArchetype,
	testimony: Option<Expression<Testimony>>,
	inner: KillData,
}

#[derive(Debug)]
pub struct KillData {
	true_identity: Option<VillagerArchetype>,
	corrupted: bool,
}

#[derive(Debug)]
pub struct SlayerKill {
	target: VillagerIndex,
	result: KillResult,
}

#[derive(Debug)]
pub struct AbilityResult {
	source: VillagerIndex,
	testimony: Option<Expression<Testimony>>,
	slayer_kill: Option<SlayerKill>,
}

#[derive(Debug)]
pub enum Action {
	TryReveal(RevealResult),
	TryExecute(KillAttempt),
	Ability(AbilityResult),
	LilisNightKill(Option<VillagerIndex>),
}

#[derive(Error, Debug)]
pub enum GameStateInitError {
	#[error("Provided villager count does not match DrawStats")]
	VillagerCountMismatch,
	#[error("Provided revealed villager count does not match reveal order count")]
	RevealOrderCountMismatch,
}

#[derive(Error, Debug)]
pub enum GameStateMutationError {
	#[error("A night action must be taken")]
	MustTakeNightAction,
	#[error("A night action cannot be taken")]
	CannotTakeNightAction,
	#[error("The target villager cannot be revealed")]
	VillagerCannotBeRevealed,
	#[error("The target villager is already dead")]
	OmaeWaMouShindeiru,
	#[error("The target cannot be unrevealed killed as it is already revealed")]
	InvalidUnrevealedKill,
	#[error("The target cannot be revealed killed as it is has not been revealed")]
	InvalidRevealedKill,
	#[error("The target cannot be killed because it has been set as unkillable")]
	InvalidUnkillableKill,
	#[error("Cannot use the ability of an unrevealed villager")]
	CannotUseAbilityOfUnrevealedVillager,
	#[error("The source's ability is not available")]
	AbilityNotAvailable,
	#[error("Lilis can't kill a revealed villager")]
	LilisCantKillRevealedVillager,
	#[error("Lilis can't kill an unkillable hidden villager")]
	LilisCantKillUnkillableVillager,
	#[error("Can't use ability on dead target")]
	AbilityTargetMustBeAlive,
	#[error("Slayer kill data was inconsistent")]
	SlayerKillDataMismatch,
	#[error("Trying to replace an existing testimony")]
	CannotReplaceTestimony,
	#[error("Trying to reveal a card that is disallowed by the deck")]
	InvalidReveal,
	#[error("A villager with an action cannot be revealed with a testimony")]
	RevealActionAndTestimony,
	#[error("A villager without an action cannot be revealed without a testimony")]
	RevealNoActionNorTestimony,
}

#[derive(Debug, PartialEq, Eq)]
pub enum GameStateMutationResult {
	Win,
	Loss,
	Continue,
}

#[derive(Error, Debug)]
pub enum KillDataConstructionError {
	#[error("The true identity cannot be corrupted")]
	TrueIdentityCannotBeCorrupted,
}

impl KillAttempt {
	pub fn new(target: VillagerIndex, result: Option<KillResult>) -> Self {
		Self { target, result }
	}

	pub fn target(&self) -> &VillagerIndex {
		&self.target
	}
}

impl KillData {
	pub fn new(
		true_identity: Option<VillagerArchetype>,
		corrupted: bool,
	) -> Result<Self, KillDataConstructionError> {
		if let Some(archetype) = &true_identity
			&& archetype.can_be_corrupted()
		{
			return Err(KillDataConstructionError::TrueIdentityCannotBeCorrupted);
		}

		Ok(Self {
			true_identity,
			corrupted,
		})
	}
}

impl SlayerKill {
	pub fn new(target: VillagerIndex, result: KillResult) -> Self {
		Self { target, result }
	}
}

impl AbilityResult {
	pub fn new(
		source: VillagerIndex,
		testimony: Option<Expression<Testimony>>,
		slayer_kill: Option<SlayerKill>,
	) -> Self {
		Self {
			source,
			testimony,
			slayer_kill,
		}
	}
}

impl RevealResult {
	pub fn new(index: VillagerIndex, instance: Option<VillagerInstance>) -> Self {
		Self { index, instance }
	}

	pub fn index(&self) -> &VillagerIndex {
		&self.index
	}
}

impl DrawStats {
	pub fn new(villagers: u8, outcasts: u8, minions: u8, demons: u8) -> DrawStats {
		Self {
			villagers,
			outcasts,
			minions,
			demons,
		}
	}

	pub fn total_villagers(&self) -> usize {
		(self.villagers + self.outcasts + self.minions + self.demons) as usize
	}

	pub fn villagers(&self) -> u8 {
		self.villagers
	}

	pub fn outcasts(&self) -> u8 {
		self.outcasts
	}

	pub fn minions(&self) -> u8 {
		self.minions
	}

	pub fn demons(&self) -> u8 {
		self.demons
	}
}

impl GameState {
	pub fn new(
		next_day: Option<u8>,
		draw_stats: DrawStats,
		deck: Vec<VillagerArchetype>,
		villagers: Vec<Villager>,
		reveal_order: Vec<VillagerIndex>,
		hitpoints: u8,
		total_evils: u8,
	) -> Result<Self, GameStateInitError> {
		if draw_stats.total_villagers() != villagers.len() {
			return Err(GameStateInitError::VillagerCountMismatch);
		}

		if villagers
			.iter()
			.map(|villager| match villager {
				Villager::Active(_) => 1,
				Villager::Hidden(_) => 0,
				Villager::Confirmed(_) => 1,
			})
			.sum::<usize>()
			!= reveal_order.len()
		{
			return Err(GameStateInitError::RevealOrderCountMismatch);
		}

		Ok(Self {
			next_day,
			draw_stats,
			deck,
			villagers,
			reveal_order,
			hitpoints,
			total_evils,
		})
	}

	pub fn deck(&self) -> &Vec<VillagerArchetype> {
		&self.deck
	}

	pub fn villager_indicies(&self) -> impl Iterator<Item = VillagerIndex> {
		self.villagers
			.iter()
			.enumerate()
			.map(|(index, _)| VillagerIndex(index))
	}

	pub fn villager(&self, index: &VillagerIndex) -> &Villager {
		&self.villagers[index.0]
	}

	pub fn villagers(&self) -> &Vec<Villager> {
		&self.villagers
	}

	pub fn iter_villagers<'a, F>(&'a self, mut f: F)
	where
		F: FnMut(VillagerIndex, &'a Villager),
	{
		for (index, villager) in self.villagers.iter().enumerate() {
			f(VillagerIndex(index), villager);
		}
	}

	pub fn total_evils(&self) -> u8 {
		self.total_evils
	}

	pub fn draw_stats(&self) -> &DrawStats {
		&self.draw_stats
	}

	pub fn total_villagers(&self) -> usize {
		self.villagers.len()
	}

	pub fn reveal_order(&self) -> &Vec<VillagerIndex> {
		&self.reveal_order
	}

	pub fn current_day(&self) -> Option<u8> {
		self.next_day.map(|day| day - 1)
	}

	pub fn night_actions_in_play(&self) -> bool {
		self.next_day.is_some()
	}

	pub fn witch_block_active(&self) -> bool {
		for villager in &self.villagers {
			match villager {
				Villager::Hidden(hidden_villager) => {
					if hidden_villager.cant_reveal() {
						return true;
					}
				}
				Villager::Confirmed(_) | Villager::Active(_) => {}
			}
		}

		false
	}

	pub fn evils_killed(&self) -> u8 {
		self.villagers
			.iter()
			.map(|villager| match villager {
				Villager::Active(_) | Villager::Hidden(_) => 0,
				Villager::Confirmed(confirmed_villager) => {
					match confirmed_villager.true_identity() {
						VillagerArchetype::GoodVillager(good_villager) => match good_villager {
							GoodVillager::Alchemist
							| GoodVillager::Architect
							| GoodVillager::Baker
							| GoodVillager::Bishop
							| GoodVillager::Confessor
							| GoodVillager::Empress
							| GoodVillager::Enlightened
							| GoodVillager::Gemcrafter
							| GoodVillager::Hunter
							| GoodVillager::Knight
							| GoodVillager::Knitter
							| GoodVillager::Lover
							| GoodVillager::Medium
							| GoodVillager::Oracle
							| GoodVillager::Poet
							| GoodVillager::Scout
							| GoodVillager::Witness
							| GoodVillager::Bard
							| GoodVillager::Dreamer
							| GoodVillager::Druid
							| GoodVillager::FortuneTeller
							| GoodVillager::Jester
							| GoodVillager::Judge
							| GoodVillager::Slayer => 0,
						},
						VillagerArchetype::Outcast(outcast) => match outcast {
							Outcast::Drunk
							| Outcast::Wretch
							| Outcast::Bombardier
							| Outcast::Doppelganger
							| Outcast::PlagueDoctor => 0,
						},
						VillagerArchetype::Minion(minion) => match minion {
							Minion::Counsellor
							| Minion::Witch
							| Minion::Minion
							| Minion::Poisoner
							| Minion::Twinion
							| Minion::Shaman
							| Minion::Puppeteer
							| Minion::Puppet => 1,
						},
						VillagerArchetype::Demon(demon) => match demon {
							Demon::Baa | Demon::Pooka | Demon::Lilis => 1,
						},
					}
				}
			})
			.sum::<u8>()
	}

	pub fn mutate(
		&mut self,
		action: Action,
	) -> Result<GameStateMutationResult, GameStateMutationError> {
		let must_be_night = match self.next_day {
			Some(next_day) => next_day > DAYS_BEFORE_NIGHT,
			None => false,
		};
		let mut health_deduction = 0;
		let mut reset_cant_kills = false;
		let mut revealed = None;

		match action {
			Action::TryReveal(result) => {
				if must_be_night {
					return Err(GameStateMutationError::MustTakeNightAction);
				}

				let target_villager = &mut self.villagers[result.index.0];
				match target_villager {
					Villager::Active(_) | Villager::Confirmed(_) => {
						return Err(GameStateMutationError::VillagerCannotBeRevealed);
					}
					Villager::Hidden(hidden_villager) => {
						if hidden_villager.cant_reveal() {
							return Err(GameStateMutationError::VillagerCannotBeRevealed);
						}

						match result.instance {
							Some(instance) => {
								if !self.valid_draw(instance.archetype()) {
									return Err(GameStateMutationError::InvalidReveal);
								}

								if instance.action_available() {
									if instance.testimony().is_some() {
										return Err(
											GameStateMutationError::RevealActionAndTestimony,
										);
									}
								} else if instance.testimony().is_none() {
									return Err(GameStateMutationError::RevealNoActionNorTestimony);
								}

								revealed = Some(result.index.clone());
								self.villagers[result.index.0] =
									Villager::Active(ActiveVillager::new(instance))
							}
							None => hidden_villager.set_cant_reveal(),
						}
					}
				}
			}
			Action::TryExecute(attempt) => {
				if must_be_night {
					return Err(GameStateMutationError::MustTakeNightAction);
				}
				let target_villager = &mut self.villagers[attempt.target.0];
				match target_villager {
					Villager::Active(active_villager) => match attempt.result {
						Some(result) => match result {
							KillResult::Unrevealed(_) => {
								return Err(GameStateMutationError::InvalidRevealedKill);
							}
							KillResult::Revealed(kill_data) => {
								let new_instance = active_villager.instance().clone();
								let confirmed_villager = ConfirmedVillager::new(
									new_instance,
									kill_data.true_identity,
									kill_data.corrupted,
								);
								match confirmed_villager.execution_result() {
									ExecutionResult::EvilKilled => {}
									ExecutionResult::SelfDestructKilled => {
										return Ok(GameStateMutationResult::Loss);
									}
									ExecutionResult::HealthDeduction(deduction) => {
										health_deduction += deduction
									}
								}

								if let VillagerArchetype::Minion(minion) =
									confirmed_villager.true_identity()
									&& minion == &Minion::Witch
								{
									reset_cant_kills = true
								}

								let _ = replace(
									target_villager,
									Villager::Confirmed(confirmed_villager),
								);
							}
						},
						None => active_villager.set_cant_kill(),
					},
					Villager::Hidden(hidden_villager) => {
						if hidden_villager.cant_kill() {
							return Err(GameStateMutationError::InvalidUnkillableKill);
						}
						if hidden_villager.dead() {
							return Err(GameStateMutationError::OmaeWaMouShindeiru);
						}
						match attempt.result {
							Some(result) => match result {
								KillResult::Unrevealed(kill_data) => {
									let new_instance = VillagerInstance::new(
										kill_data.identity,
										kill_data.testimony,
									);

									if valid_draw(&self.deck, new_instance.archetype()) {
										return Err(GameStateMutationError::InvalidReveal);
									}

									if new_instance.action_available() {
										if new_instance.testimony().is_some() {
											return Err(
												GameStateMutationError::RevealActionAndTestimony,
											);
										}
									} else if new_instance.testimony().is_none() {
										return Err(
											GameStateMutationError::RevealNoActionNorTestimony,
										);
									}
									let confirmed_villager = ConfirmedVillager::new(
										new_instance,
										kill_data.inner.true_identity,
										kill_data.inner.corrupted,
									);
									match confirmed_villager.execution_result() {
										ExecutionResult::EvilKilled => {}
										ExecutionResult::SelfDestructKilled => {
											return Ok(GameStateMutationResult::Loss);
										}
										ExecutionResult::HealthDeduction(deduction) => {
											health_deduction += deduction
										}
									}

									if let VillagerArchetype::Minion(minion) =
										confirmed_villager.true_identity()
										&& minion == &Minion::Witch
									{
										reset_cant_kills = true
									}
									revealed = Some(attempt.target.clone());
									let _ = replace(
										target_villager,
										Villager::Confirmed(confirmed_villager),
									);
								}
								KillResult::Revealed(_) => {
									return Err(GameStateMutationError::InvalidUnrevealedKill);
								}
							},
							None => hidden_villager.set_cant_kill(),
						}
					}
					Villager::Confirmed(_) => {
						return Err(GameStateMutationError::OmaeWaMouShindeiru);
					}
				}
			}
			Action::Ability(result) => {
				if must_be_night {
					return Err(GameStateMutationError::MustTakeNightAction);
				}

				let source_villager = &mut self.villagers[result.source.0];
				let instance;
				match source_villager {
					Villager::Active(active_villager) => {
						instance = active_villager.instance_mut();
					}
					Villager::Confirmed(confirmed_villager) => {
						instance = confirmed_villager.instance_mut();
					}
					Villager::Hidden(_) => {
						return Err(GameStateMutationError::CannotUseAbilityOfUnrevealedVillager);
					}
				}

				if !instance.action_available() {
					return Err(GameStateMutationError::AbilityNotAvailable);
				}

				let mut slayer_kill = None;
				match result.testimony {
					Some(testimony) => {
						if instance.testimony().is_some() {
							return Err(GameStateMutationError::CannotReplaceTestimony);
						}

						if let Expression::<Testimony>::Leaf(unary_testimony) = &testimony
							&& let Testimony::Slayed(slay_testimony) = unary_testimony
						{
							match result.slayer_kill {
								Some(local_slayer_kill) => {
									if local_slayer_kill.target != *slay_testimony.index() {
										return Err(GameStateMutationError::SlayerKillDataMismatch);
									}

									slayer_kill = Some(local_slayer_kill);
								}
								None => {
									return Err(GameStateMutationError::SlayerKillDataMismatch);
								}
							}
						}

						// slayer validation
						if let Some(slayer_kill) = &slayer_kill {
							let target_villager = &mut self.villagers[slayer_kill.target.0];
							match target_villager {
								Villager::Active(active_villager) => {
									// safe to do this here, slayer can only kill once
									match &slayer_kill.result {
										KillResult::Unrevealed(_) => {
											return Err(
												GameStateMutationError::InvalidRevealedKill,
											);
										}
										KillResult::Revealed(kill_data) => {
											let new_instance = active_villager.instance().clone();

											let confirmed_villager = ConfirmedVillager::new(
												new_instance,
												kill_data.true_identity.clone(),
												kill_data.corrupted,
											);
											if let VillagerArchetype::Minion(minion) =
												confirmed_villager.true_identity() && minion
												== &Minion::Witch
											{
												reset_cant_kills = true
											}
											let _ = replace(
												target_villager,
												Villager::Confirmed(confirmed_villager),
											);
										}
									}
								}
								Villager::Hidden(hidden_villager) => {
									if hidden_villager.dead() {
										return Err(GameStateMutationError::OmaeWaMouShindeiru);
									}

									// safe to do this here, slayer can only kill once
									match &slayer_kill.result {
										KillResult::Unrevealed(kill_data) => {
											let new_instance = VillagerInstance::new(
												kill_data.identity.clone(),
												kill_data.testimony.clone(),
											);
											if valid_draw(&self.deck, new_instance.archetype()) {
												return Err(GameStateMutationError::InvalidReveal);
											}

											if new_instance.action_available() {
												if new_instance.testimony().is_some() {
													return Err(GameStateMutationError::RevealActionAndTestimony);
												}
											} else if new_instance.testimony().is_none() {
												return Err(GameStateMutationError::RevealNoActionNorTestimony);
											}

											let confirmed_villager = ConfirmedVillager::new(
												new_instance,
												kill_data.inner.true_identity.clone(),
												kill_data.inner.corrupted,
											);
											if let VillagerArchetype::Minion(minion) =
												confirmed_villager.true_identity() && minion
												== &Minion::Witch
											{
												reset_cant_kills = true
											}
											revealed = Some(slayer_kill.target.clone());
											let _ = replace(
												target_villager,
												Villager::Confirmed(confirmed_villager),
											);
										}
										KillResult::Revealed(_) => {
											return Err(
												GameStateMutationError::InvalidUnrevealedKill,
											);
										}
									}
								}
								Villager::Confirmed(_) => {
									return Err(GameStateMutationError::OmaeWaMouShindeiru);
								}
							}
						}

						// TODO: Validate more testimonies if it makes sense to do so
						match &mut self.villagers[result.source.0] {
							Villager::Active(active_villager) => {
								active_villager.instance_mut().set_testimony(testimony)
							}
							Villager::Confirmed(confirmed_villager) => {
								confirmed_villager.instance_mut().set_testimony(testimony)
							}
							Villager::Hidden(_) => {
								panic!("Source villager should not have become a hidden villager!")
							}
						};
					}
					None => {
						if result.slayer_kill.is_some() {
							return Err(GameStateMutationError::SlayerKillDataMismatch);
						}
					}
				}
			}
			Action::LilisNightKill(villager_index) => {
				if !must_be_night {
					return Err(GameStateMutationError::CannotTakeNightAction);
				}
				if let Some(index) = villager_index {
					let target_villager = &mut self.villagers[index.0];
					match target_villager {
						Villager::Active(_) => {
							return Err(GameStateMutationError::LilisCantKillRevealedVillager);
						}
						Villager::Hidden(hidden_villager) => {
							if hidden_villager.dead() {
								return Err(GameStateMutationError::OmaeWaMouShindeiru);
							}

							if hidden_villager.cant_kill() {
								return Err(
									GameStateMutationError::LilisCantKillUnkillableVillager,
								);
							}

							hidden_villager.die();
						}
						Villager::Confirmed(_) => {
							return Err(GameStateMutationError::OmaeWaMouShindeiru);
						}
					}
				}
			}
		};

		if let Some(revealed) = revealed {
			self.reveal_order.push(revealed);
		}

		if reset_cant_kills {
			for villager in &mut self.villagers {
				if let Villager::Hidden(hidden_villager) = villager {
					hidden_villager.reset_cant_reveal();
				}
			}
		}

		if health_deduction >= self.hitpoints {
			self.hitpoints = 0;
			return Ok(GameStateMutationResult::Loss);
		}

		self.hitpoints -= health_deduction;

		if self.evils_killed() >= self.total_evils {
			return Ok(GameStateMutationResult::Win);
		}

		Ok(GameStateMutationResult::Continue)
	}

	pub fn valid_draw(&self, archetype: &VillagerArchetype) -> bool {
		valid_draw(&self.deck, archetype)
	}
}

pub fn new_game(
	deck: Vec<VillagerArchetype>,
	draw_stats: DrawStats,
	total_evils: u8,
	night_effects_active: bool,
) -> GameState {
	let total_villagers = draw_stats.total_villagers();
	GameState::new(
		if night_effects_active { Some(1) } else { None },
		draw_stats,
		deck,
		std::iter::repeat_n(0, total_villagers)
			.map(|_| Villager::Hidden(HiddenVillager::new(false, false, false)))
			.collect(),
		Vec::new(),
		10,
		total_evils,
	)
	.expect("logic error in new_game creation")
}

fn valid_draw(deck: &Vec<VillagerArchetype>, archetype: &VillagerArchetype) -> bool {
	let prereq = archetype.deck_prerequisite();
	for deck_archetype in deck {
		if *deck_archetype == prereq {
			return true;
		}
	}

	false
}
