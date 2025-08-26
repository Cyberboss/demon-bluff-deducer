use std::arch::breakpoint;

use demon_bluff_gameplay_engine::{game_state::GameState, villager::VillagerIndex};
use log::Log;

use super::{
    HypothesisBuilderType,
    desires::{DesireType, GetTestimonyDesire},
};

use crate::{
    engine::{
        Depth, DesireConsumerReference, FitnessAndAction, Hypothesis, HypothesisBuilder,
        HypothesisEvaluation, HypothesisRegistrar, HypothesisRepository, HypothesisResult,
    },
    hypotheses::HypothesisType,
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct NeedTestimonyHypothesisBuilder {
    index: VillagerIndex,
}

#[derive(Debug)]
pub struct NeedTestimonyHypothesis {
    index: VillagerIndex,
    get_testimony_desire: DesireConsumerReference,
}

impl NeedTestimonyHypothesisBuilder {
    pub fn new(index: VillagerIndex) -> Self {
        Self { index }
    }
}

impl HypothesisBuilder for NeedTestimonyHypothesisBuilder {
    fn build(
        self,
        game_state: &GameState,
        registrar: &mut impl HypothesisRegistrar<HypothesisBuilderType, DesireType>,
    ) -> HypothesisType {
        let get_testimony_desire = registrar.register_desire_consumer(DesireType::GetTestimony(
            GetTestimonyDesire::new(self.index.clone()),
        ));
        NeedTestimonyHypothesis {
            index: self.index,
            get_testimony_desire,
        }
        .into()
    }
}

impl Hypothesis for NeedTestimonyHypothesis {
    fn describe(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "Need testimony of {}", self.index)
    }

    fn evaluate(
        &mut self,
        log: &impl Log,
        depth: Depth,
        game_state: &GameState,
        repository: impl HypothesisRepository,
    ) -> HypothesisEvaluation {
        let result = repository.desire_result(&self.get_testimony_desire);
        repository.finalize(result)
    }
}
