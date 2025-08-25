use std::arch::breakpoint;

use demon_bluff_gameplay_engine::{game_state::GameState, villager::VillagerIndex};
use log::Log;

use crate::{
    desires::{DesireType, get_testimony::GetTestimonyDesire},
    engine::{
        Depth, DesireConsumerReference, FitnessAndAction, Hypothesis, HypothesisBuilder,
        HypothesisRegistrar, HypothesisRepository, HypothesisResult, HypothesisReturn,
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
    fn build<TLog>(self, _: &GameState, registrar: &mut HypothesisRegistrar<TLog>) -> HypothesisType
    where
        TLog: ::log::Log,
    {
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
        let result = repository.desire_result(&self.get_testimony_desire);
        repository.create_return(result)
    }
}
