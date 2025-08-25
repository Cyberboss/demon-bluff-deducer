use demon_bluff_gameplay_engine::{
    Expression,
    game_state::GameState,
    testimony::{self, Testimony},
    villager::VillagerIndex,
};
use log::Log;

use crate::engine::{
    Depth, FITNESS_UNKNOWN, FitnessAndAction, Hypothesis, HypothesisBuilder, HypothesisReference,
    HypothesisRegistrar, HypothesisRepository, HypothesisResult, HypothesisReturn,
};

use super::HypothesisType;

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct TestimonyCondemnsHypothesisBuilder {
    testifier: VillagerIndex,
    defendant: VillagerIndex,
    testimony: Expression<Testimony>,
}

impl TestimonyCondemnsHypothesisBuilder {
    pub fn new(
        testifier: VillagerIndex,
        testimony: Expression<Testimony>,
        defendant: VillagerIndex,
    ) -> Self {
        Self {
            testifier,
            defendant,
            testimony,
        }
    }
}

/// If a testimony is true and condemns a given defendent
#[derive(Debug)]
pub struct TestimonyCondemnsHypothesis {
    testifier: VillagerIndex,
    defendant: VillagerIndex,
}

impl HypothesisBuilder for TestimonyCondemnsHypothesisBuilder {
    fn build<TLog>(self, _: &GameState, registrar: &mut HypothesisRegistrar<TLog>) -> HypothesisType
    where
        TLog: ::log::Log,
    {
        TestimonyCondemnsHypothesis {
            testifier: self.testifier,
            defendant: self.defendant,
        }
        .into()
    }
}

impl Hypothesis for TestimonyCondemnsHypothesis {
    fn describe(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}'s testimony condemns {}",
            self.testifier, self.defendant
        )
    }

    fn wip(&self) -> bool {
        true
    }

    fn evaluate<TLog>(
        &mut self,
        _: &TLog,
        _: Depth,
        _: &GameState,
        repository: HypothesisRepository<TLog>,
    ) -> HypothesisReturn
    where
        TLog: Log,
    {
        repository.create_return(HypothesisResult::Conclusive(FitnessAndAction::certainty(
            None,
        )))
    }
}
