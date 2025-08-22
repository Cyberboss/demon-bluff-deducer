use std::collections::HashMap;

use demon_bluff_gameplay_engine::{
    game_state::GameState,
    villager::{Villager, VillagerIndex},
};
use log::Log;

use crate::hypothesis::{
    Depth, Hypothesis, HypothesisReference, HypothesisRegistrar, HypothesisRepository,
    HypothesisReturn,
};

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
    fn describe(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "Reveal a Villager")
    }

    fn evaluate<TLog>(
        &mut self,
        log: &TLog,
        depth: Depth,
        game_state: &GameState,
        repository: HypothesisRepository<TLog>,
    ) -> HypothesisReturn
    where
        TLog: Log,
    {
        todo!()
    }
}
