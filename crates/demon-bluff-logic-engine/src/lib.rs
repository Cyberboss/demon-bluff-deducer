use std::collections::HashSet;

use demon_bluff_gameplay_engine::{
    game_state::GameState,
    villager::{GoodVillager, Villager, VillagerArchetype, VillagerIndex},
};
use thiserror::Error;

#[derive(Debug, Eq)]
pub struct AbilityAttempt {
    source: VillagerIndex,
    targets: HashSet<VillagerIndex>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum PlayerAction {
    TryReveal(VillagerIndex),
    TryExecute(VillagerIndex),
    Ability(AbilityAttempt),
}

#[derive(Error, Debug)]
pub enum PredictionError {}

impl PartialEq for AbilityAttempt {
    fn eq(&self, other: &Self) -> bool {
        self.source == other.source
            && self.targets.len() == other.targets.len()
            && self
                .targets
                .iter()
                .all(|target| other.targets.contains(target))
    }
}

pub fn predict(state: &GameState) -> Result<PlayerAction, PredictionError> {
    let mut hidden_villager_index = None;
    let mut baker_present = false;

    let mut revealed_villagers = 0;
    state.iter_villagers(|index, villager| match villager {
        Villager::Active(active_villager) => {
            revealed_villagers = revealed_villagers + 1;
            if *active_villager.instance().archetype()
                == VillagerArchetype::GoodVillager(GoodVillager::Baker)
            {
                baker_present = true;
            }
        }
        Villager::Hidden(hidden_villager) => {
            if hidden_villager_index.is_none() && !hidden_villager.cant_reveal() {
                hidden_villager_index = Some(index)
            }
        }
        Villager::Confirmed(confirmed_villager) => {
            revealed_villagers = revealed_villagers + 1;
            if *confirmed_villager.instance().archetype()
                == VillagerArchetype::GoodVillager(GoodVillager::Baker)
            {
                baker_present = true;
            }
        }
    });

    if let Some(hidden_villager_index) = hidden_villager_index {
        if revealed_villagers > 0
            && (baker_present || state.witch_block_active() || todo!("Witch active deduction"))
        {
            todo!("What do I do when revealing causes harm?");
        }

        return Ok(PlayerAction::TryReveal(hidden_villager_index));
    }

    todo!()
}
