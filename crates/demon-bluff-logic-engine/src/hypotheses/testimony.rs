use demon_bluff_gameplay_engine::{
    game_state::GameState,
    testimony::{self, Testimony},
    villager::VillagerIndex,
};
use log::Log;

use crate::{
    engine::{
        Depth, FitnessAndAction, Hypothesis, HypothesisBuilder, HypothesisEvaluation,
        HypothesisReference, HypothesisRegistrar, HypothesisRepository, HypothesisResult,
    },
    hypotheses::{HypothesisType, testimony_expression::TestimonyExpressionHypothesis},
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
    fn build(
        self,
        game_state: &GameState,
        registrar: &mut impl HypothesisRegistrar<HypothesisBuilderType, DesireType>,
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

    fn evaluate(
        &mut self,
        log: &impl Log,
        depth: Depth,
        game_state: &GameState,
        repository: impl HypothesisRepository,
    ) -> HypothesisEvaluation {
        match &self.testimony {
            Testimony::Confess(confessor_claim) => repository.finalize(
                HypothesisResult::Conclusive(FitnessAndAction::certainty(None)),
            ),
            _ => repository.finalize(HypothesisResult::unimplemented()),
        }
    }
}
