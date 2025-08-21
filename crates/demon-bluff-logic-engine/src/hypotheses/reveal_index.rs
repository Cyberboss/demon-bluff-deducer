use std::collections::HashSet;

use demon_bluff_gameplay_engine::{
    game_state::{self, GameState},
    villager::VillagerIndex,
};

use crate::{
    hypothesis::{Hypothesis, HypothesisReference, HypothesisRegistrar, HypothesisRepository},
    player_action::PlayerAction,
};

struct RevealIndexHypothesis {
    index: VillagerIndex,
}

impl RevealIndexHypothesis {
    pub fn create(
        game_state: &GameState,
        registrar: &mut HypothesisRegistrar,
        index: VillagerIndex,
    ) -> HypothesisReference {
        registrar.register(Box::new(Self { index }))
    }
}

impl Hypothesis for RevealIndexHypothesis {
    fn evaluate(
        &mut self,
        game_state: &GameState,
        repository: &mut HypothesisRepository,
    ) -> Option<f64> {
        todo!()
    }

    fn action(&self) -> HashSet<PlayerAction> {
        let mut map = HashSet::new();
        map.insert(PlayerAction::TryReveal(self.index));
        map
    }
}
