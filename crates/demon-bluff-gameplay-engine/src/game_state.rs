use std::iter::repeat;

use crate::villager::{ConfirmedVillager, HiddenVillager, RevealedVillager, VillagerArchetype};

pub enum DayCycle {
    Day1,
    Day2,
    Day3,
    Day4,
}

pub struct DrawStats {
    villagers: u8,
    outcasts: u8,
    minions: u8,
    demons: u8,
}

pub struct GameState {
    day_cycle: DayCycle,
    draw_stats: DrawStats,
    deck: Vec<VillagerArchetype>,
    revealed_villagers: Vec<RevealedVillager>,
    hidden_villagers: Vec<HiddenVillager>,
    confirmed_villagers: Vec<ConfirmedVillager>,
    hitpoints: u8,
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

    pub fn total_villagers(&self) -> u8 {
        self.villagers + self.outcasts + self.minions + self.demons
    }
}

impl GameState {
    pub fn new(
        day_cycle: DayCycle,
        draw_stats: DrawStats,
        deck: Vec<VillagerArchetype>,
        revealed_villagers: Vec<RevealedVillager>,
        hidden_villagers: Vec<HiddenVillager>,
        confirmed_villagers: Vec<ConfirmedVillager>,
        hitpoints: u8,
    ) -> Result<Self, ()> {
        Ok(Self {
            day_cycle,
            draw_stats,
            deck,
            revealed_villagers,
            hidden_villagers,
            confirmed_villagers,
            hitpoints,
        })
    }
}

pub fn new_game(deck: Vec<VillagerArchetype>, draw_stats: DrawStats) -> GameState {
    let total_villagers = draw_stats.total_villagers();
    GameState::new(
        DayCycle::Day1,
        draw_stats,
        deck,
        Vec::new(),
        repeat(0)
            .take(total_villagers as usize)
            .map(|_| HiddenVillager::new(false))
            .collect(),
        Vec::new(),
        10,
    )
    .expect("logic error in new_game creation")
}
