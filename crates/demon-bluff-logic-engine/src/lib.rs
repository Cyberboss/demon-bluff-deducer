mod hypotheses;
mod hypothesis;
pub mod player_action;

use std::collections::HashSet;

use demon_bluff_gameplay_engine::{
    game_state::GameState,
    villager::{GoodVillager, Villager, VillagerArchetype, VillagerIndex},
};
use player_action::PlayerAction;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PredictionError {}

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
