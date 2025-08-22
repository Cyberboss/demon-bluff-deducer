use demon_bluff_gameplay_engine::{game_state::GameState, villager::VillagerIndex};

use crate::hypothesis::{
    Hypothesis, HypothesisReference, HypothesisRegistrar, HypothesisRepository, HypothesisReturn,
};

#[derive(Eq, PartialEq, Debug)]
pub struct RevealIndexHypothesis {
    index: VillagerIndex,
}

impl RevealIndexHypothesis {
    pub fn create(
        _: &GameState,
        registrar: &mut HypothesisRegistrar,
        index: VillagerIndex,
    ) -> HypothesisReference {
        registrar.register(Self { index })
    }
}

impl Hypothesis for RevealIndexHypothesis {
    fn describe(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "Reveal Villager #{}", self.index.0 + 1)
    }

    fn evaluate(
        &mut self,
        game_state: &GameState,
        repository: &mut HypothesisRepository,
    ) -> HypothesisReturn {
        todo!()
    }
}
