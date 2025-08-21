use std::ffi::os_str::Display;

use demon_bluff_gameplay_engine::game_state::{self, GameState};

use crate::{
    hypothesis::{Hypothesis, HypothesisReference, HypothesisRegistrar, HypothesisRepository},
    player_action::PlayerAction,
};

use super::reveal::RevealHypothesis;

struct MasterHypothesis {
    reveal: HypothesisReference,
    execute: HypothesisReference,
    ability: HypothesisReference,
}

impl MasterHypothesis {
    pub fn create(
        game_state: &GameState,
        registrar: &mut HypothesisRegistrar,
    ) -> HypothesisReference {
        registrar.register(Box::new(Self {
            reveal: RevealHypothesis::create(game_state, registrar),
            execute: todo!(),
            ability: todo!(),
        }))
    }
}

impl Hypothesis for MasterHypothesis {
    fn evaluate(
        &mut self,
        game_state: &GameState,
        repository: &mut HypothesisRepository,
    ) -> Option<f64> {
        todo!()
    }

    fn action(&self) -> std::collections::HashSet<PlayerAction> {
        todo!()
    }
}
