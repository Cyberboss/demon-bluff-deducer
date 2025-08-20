use std::{iter::repeat, mem::replace};

use thiserror::Error;

use crate::{
    Expression,
    testimony::Testimony,
    villager::{
        ActiveVillager, ConfirmedVillager, HiddenVillager, Villager, VillagerArchetype,
        VillagerIndex, VillagerInstance,
    },
};

const DAYS_BEFORE_NIGHT: u8 = 4;

pub enum DayCycle {
    Day1,
    Day2,
    Day3,
    Day4,
    Night,
}

pub struct DrawStats {
    villagers: u8,
    outcasts: u8,
    minions: u8,
    demons: u8,
}

pub struct GameState {
    next_day: u8,
    draw_stats: DrawStats,
    deck: Vec<VillagerArchetype>,
    villagers: Vec<Villager>,
    reveal_order: Vec<VillagerIndex>,
    hitpoints: u8,
}

pub struct RevealResult {
    index: VillagerIndex,
    instance: Option<VillagerInstance>,
}

pub struct KillAttempt {
    target: VillagerIndex,
    result: Option<KillResult>,
}

pub enum KillResult {
    Unrevealed(UnrevealedKillData),
    Revealed(KillData),
}

pub struct UnrevealedKillData {
    identity: VillagerArchetype,
    testimony: Option<Expression<Testimony>>,
    inner: KillData,
}

pub struct KillData {
    true_identity: Option<VillagerArchetype>,
    corrupted: bool,
}

pub struct AbilityResult<'a> {
    source: VillagerIndex,
    targets: &'a [VillagerIndex],
    testimony: Expression<Testimony>,
}

pub enum Action<'a> {
    TryReveal(RevealResult),
    TryKill(KillAttempt),
    Ability(AbilityResult<'a>),
    LilisNightKill(VillagerIndex),
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
    CantKill,
}

impl GameState {
    pub fn new(
        next_day: u8,
        draw_stats: DrawStats,
        deck: Vec<VillagerArchetype>,
        villagers: Vec<Villager>,
        reveal_order: Vec<VillagerIndex>,
        hitpoints: u8,
    ) -> Result<Self, GameStateInitError> {
        if draw_stats.total_villagers() != villagers.len() {
            return Err(GameStateInitError::VillagerCountMismatch);
        }

        if villagers
            .iter()
            .map(|villager| match villager {
                Villager::Active(_) => 0,
                Villager::Hidden(_) => 1,
                Villager::Confirmed(_) => 0,
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
        })
    }

    pub fn mutate(&mut self, action: Action) -> Result<(), GameStateMutationError> {
        let must_be_night = self.next_day > DAYS_BEFORE_NIGHT;
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
                                self.villagers[result.index.0] =
                                    Villager::Active(ActiveVillager::new(instance))
                            }
                            None => hidden_villager.set_cant_reveal(),
                        }
                    }
                }
            }
            Action::TryKill(attempt) => {
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
                                let _ = replace(
                                    target_villager,
                                    Villager::Confirmed(ConfirmedVillager::new(
                                        new_instance,
                                        kill_data.true_identity,
                                        kill_data.corrupted,
                                    )),
                                );
                            }
                        },
                        None => active_villager.set_cant_kill(),
                    },
                    Villager::Hidden(hidden_villager) => {
                        if hidden_villager.cant_kill() {
                            return Err(GameStateMutationError::CantKill);
                        }
                        match attempt.result {
                            Some(result) => match result {
                                KillResult::Unrevealed(kill_data) => {
                                    let new_instance = VillagerInstance::new(
                                        kill_data.identity,
                                        kill_data.testimony,
                                    );
                                    let _ = replace(
                                        target_villager,
                                        Villager::Confirmed(ConfirmedVillager::new(
                                            new_instance,
                                            kill_data.inner.true_identity,
                                            kill_data.inner.corrupted,
                                        )),
                                    );
                                }
                                KillResult::Revealed(_) => {
                                    return Err(GameStateMutationError::InvalidUnrevealedKill);
                                }
                            },
                            None => hidden_villager.set_cant_kill(),
                        }
                    }
                    Villager::Confirmed(confirmed_villager) => {
                        return Err(GameStateMutationError::OmaeWaMouShindeiru);
                    }
                }
                todo!();
            }
            Action::Ability(result) => {
                if must_be_night {
                    return Err(GameStateMutationError::MustTakeNightAction);
                }

                todo!();
            }
            Action::LilisNightKill(villager_index) => {
                if !must_be_night {
                    return Err(GameStateMutationError::CannotTakeNightAction);
                }
            }
        };

        return Ok(());
    }
}

pub fn new_game(deck: Vec<VillagerArchetype>, draw_stats: DrawStats) -> GameState {
    let total_villagers = draw_stats.total_villagers();
    GameState::new(
        1,
        draw_stats,
        deck,
        repeat(0)
            .take(total_villagers)
            .map(|_| Villager::Hidden(HiddenVillager::new(false, false)))
            .collect(),
        Vec::new(),
        10,
    )
    .expect("logic error in new_game creation")
}
