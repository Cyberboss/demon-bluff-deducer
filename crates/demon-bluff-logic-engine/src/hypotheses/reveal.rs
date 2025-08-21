use std::collections::HashMap;

use demon_bluff_gameplay_engine::{
    game_state::GameState,
    villager::{Villager, VillagerIndex},
};

use crate::hypothesis::{Hypothesis, HypothesisReference, HypothesisRegistrar};

use super::reveal_index::RevealIndexHypothesis;

#[derive(Eq, PartialEq, Debug)]
pub struct RevealHypothesis {
    revealable_indexes: HashMap<VillagerIndex, HypothesisReference>,
}

impl RevealHypothesis {
    pub fn create(
        game_state: &GameState,
        registrar: &mut HypothesisRegistrar,
    ) -> HypothesisReference {
        let mut revealable_indexes = HashMap::new();
        game_state.iter_villagers(|villager_index, villager| match villager {
            Villager::Active(_) | Villager::Confirmed(_) => {}
            Villager::Hidden(hidden_villager) => {
                if !hidden_villager.cant_reveal() {
                    revealable_indexes.insert(
                        villager_index.clone(),
                        RevealIndexHypothesis::create(game_state, registrar, villager_index),
                    );
                }
            }
        });

        registrar.register(Self { revealable_indexes })
    }
}

impl Hypothesis for RevealHypothesis {
    fn evaluate(
        &mut self,
        game_state: &demon_bluff_gameplay_engine::game_state::GameState,
        repository: &mut crate::hypothesis::HypothesisRepository,
    ) -> crate::hypothesis::HypothesisReturn {
        todo!()
    }
}
