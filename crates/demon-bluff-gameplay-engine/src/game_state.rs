use std::iter::repeat;

use crate::villager::{
    ConfirmedVillager, HiddenVillager, RevealedVillager, Villager, VillagerArchetype, VillagerIndex,
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
    archetype: VillagerArchetype,
}

pub enum Action<'a> {
    Reveal(RevealResult),
    CantReveal(VillagerIndex),
    Kill(VillagerIndex),
    Ability {
        source: VillagerIndex,
        targets: &'a [VillagerIndex],
    },
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

impl GameState {
    pub fn new(
        next_day: u8,
        draw_stats: DrawStats,
        deck: Vec<VillagerArchetype>,
        villagers: Vec<Villager>,
        reveal_order: Vec<VillagerIndex>,
        hitpoints: u8,
    ) -> Result<Self, ()> {
        if draw_stats.total_villagers() != villagers.len() {
            return Err(());
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

    pub fn mutate(&mut self, action: Action) -> Result<(), ()> {
        let must_be_night = self.next_day > DAYS_BEFORE_NIGHT;
        match action {
            Action::Reveal(result) => {
                if must_be_night {
                    return Err(());
                }

                todo!();
            }
            Action::Kill(villager_index) => {
                if must_be_night {
                    return Err(());
                }

                todo!();
            }
            Action::Ability { source, targets } => {
                if must_be_night {
                    return Err(());
                }

                todo!();
            }
            Action::LilisNightKill(villager_index) => {
                if !must_be_night {
                    return Err(());
                }
            }
            Action::CantReveal(villager_index) => {
                if must_be_night {
                    return Err(());
                }

                todo!();
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
