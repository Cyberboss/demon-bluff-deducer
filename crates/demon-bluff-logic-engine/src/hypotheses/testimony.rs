use demon_bluff_gameplay_engine::{
    game_state::GameState, testimony::Testimony, villager::VillagerIndex,
};
use log::Log;

use crate::{
    engine::{
        Depth, FitnessAndAction, Hypothesis, HypothesisBuilder, HypothesisEvaluation,
        HypothesisFunctions, HypothesisRegistrar, HypothesisRepository, HypothesisResult,
    },
    hypotheses::HypothesisType,
};

use super::{DesireType, HypothesisBuilderType};

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct TestimonyHypothesisBuilder {
    index: VillagerIndex,
    testimony: Testimony,
}

/// Checks the validity of a [`Testimony`]` from a [`VillagerIndex`]
#[derive(Debug)]
pub struct TestimonyHypothesis {
    index: VillagerIndex,
    testimony: Testimony,
}

impl TestimonyHypothesisBuilder {
    pub fn new(index: VillagerIndex, testimony: Testimony) -> Self {
        Self { index, testimony }
    }
}

impl HypothesisBuilder for TestimonyHypothesisBuilder {
    fn build(
        self,
        _: &GameState,
        _: &mut impl HypothesisRegistrar<HypothesisBuilderType, DesireType>,
    ) -> HypothesisType {
        TestimonyHypothesis {
            index: self.index,
            testimony: self.testimony,
        }
        .into()
    }
}

impl Hypothesis for TestimonyHypothesis {
    fn describe(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "Testimony of {} is true: {}", self.index, self.testimony)
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
    ) -> HypothesisEvaluation
    where
        TLog: Log,
    {
        match &self.testimony {
            Testimony::Confess(_) => repository.finalize(HypothesisResult::Conclusive(
                FitnessAndAction::certainty(None),
            )),
            _ => repository.finalize(HypothesisResult::unimplemented()),
        }
    }
}
