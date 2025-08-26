use demon_bluff_gameplay_engine::{
    Expression,
    game_state::GameState,
    testimony::{self, Testimony},
    villager::VillagerIndex,
};
use log::Log;

use crate::engine::{
    Depth, FITNESS_UNKNOWN, FitnessAndAction, Hypothesis, HypothesisBuilder, HypothesisEvaluation,
    HypothesisReference, HypothesisRegistrar, HypothesisRepository, HypothesisResult,
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
    fn build(
        self,
        game_state: &GameState,
        registrar: &mut impl HypothesisRegistrar<HypothesisBuilderType, DesireType>,
    ) -> HypothesisType {
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

    fn evaluate(
        &mut self,
        log: &impl Log,
        depth: Depth,
        game_state: &GameState,
        repository: impl HypothesisRepository,
    ) -> HypothesisEvaluation {
        repository.finalize(HypothesisResult::unimplemented())
    }
}
