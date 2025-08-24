use demon_bluff_gameplay_engine::{
    game_state::GameState,
    testimony::{self, Testimony},
    villager::VillagerIndex,
};
use log::Log;

use crate::{
    hypotheses::{HypothesisType, testimony_expression::TestimonyExpressionHypothesis},
    hypothesis::{
        Depth, FitnessAndAction, Hypothesis, HypothesisBuilder, HypothesisReference,
        HypothesisRegistrar, HypothesisRepository, HypothesisResult, HypothesisReturn,
    },
};

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
    fn build<TLog>(
        self,
        game_state: &GameState,
        registrar: &mut HypothesisRegistrar<TLog>,
    ) -> HypothesisType
    where
        TLog: ::log::Log,
    {
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
        log: &TLog,
        depth: Depth,
        game_state: &GameState,
        repository: HypothesisRepository<TLog>,
    ) -> HypothesisReturn
    where
        TLog: Log,
    {
        match &self.testimony {
            Testimony::Confess(confessor_claim) => repository.create_return(
                HypothesisResult::Conclusive(FitnessAndAction::certainty(None)),
            ),
            _ => repository.create_return(HypothesisResult::unimplemented()),
        }
    }
}
