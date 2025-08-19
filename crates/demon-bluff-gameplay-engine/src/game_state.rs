use crate::villager::{HiddenVillager, RevealedVillager, VillagerArchetype};

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

pub fn new_game(deck: Vec<VillagerArchetype>, draw_stats: DrawStats) -> GameState {
    todo!()
}
