use demon_bluff_gameplay_engine::{
    game_state::GameState, testimony::Testimony, villager::VillagerIndex,
};
use log::Log;

use crate::hypothesis::{
    Depth, FitnessAndAction, Hypothesis, HypothesisReference, HypothesisRegistrar,
    HypothesisRepository, HypothesisResult, HypothesisReturn,
};

#[derive(Eq, PartialEq, Debug)]
pub struct TestimonyHypothesis {
    testimony: Testimony,
}

impl TestimonyHypothesis {
    pub fn create<TLog>(
        game_state: &GameState,
        mut registrar: &mut HypothesisRegistrar<TLog>,
        testimony: Testimony,
    ) -> HypothesisReference
    where
        TLog: Log,
    {
        registrar.register(Self { testimony })
    }
}

impl Hypothesis for TestimonyHypothesis {
    fn describe(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "Testimony is true: {}", self.testimony)
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
        repository.create_return(HypothesisResult::Conclusive(
            FitnessAndAction::unimplemented(),
        ))
    }
}
