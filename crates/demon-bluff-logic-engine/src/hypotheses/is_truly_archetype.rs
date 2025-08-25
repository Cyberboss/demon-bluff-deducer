use demon_bluff_gameplay_engine::{
    game_state::GameState,
    villager::{VillagerArchetype, VillagerIndex},
};
use log::Log;

use crate::engine_old::{
    Depth, FITNESS_UNKNOWN, Hypothesis, HypothesisBuilder, HypothesisReference,
    HypothesisRegistrar, HypothesisRepository, HypothesisResult, HypothesisReturn, and_result,
};

use super::{
    HypothesisType,
    archetype_in_play::{self, ArchetypeInPlayHypothesisBuilder},
};

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct IsTrulyArchetypeHypothesisBuilder {
    archetype: VillagerArchetype,
    index: VillagerIndex,
}

impl IsTrulyArchetypeHypothesisBuilder {
    pub fn new(archetype: VillagerArchetype, index: VillagerIndex) -> Self {
        Self { archetype, index }
    }
}

#[derive(Debug)]
pub struct IsTrulyArchetypeHypothesis {
    archetype: VillagerArchetype,
    index: VillagerIndex,
    archetype_in_play_hypothesis: HypothesisReference,
}

impl HypothesisBuilder for IsTrulyArchetypeHypothesisBuilder {
    fn build<TLog>(self, _: &GameState, registrar: &mut HypothesisRegistrar<TLog>) -> HypothesisType
    where
        TLog: ::log::Log,
    {
        let archetype_in_play_hypothesis = registrar.register(
            ArchetypeInPlayHypothesisBuilder::new(self.archetype.clone()),
        );

        IsTrulyArchetypeHypothesis {
            archetype: self.archetype,
            index: self.index,
            archetype_in_play_hypothesis,
        }
        .into()
    }
}

impl Hypothesis for IsTrulyArchetypeHypothesis {
    fn describe(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{} is a {}", self.index, self.archetype)
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
        let mut evaluator = repository.require_sub_evaluation(FITNESS_UNKNOWN);

        let archetype_in_play_result = evaluator.sub_evaluate(&self.archetype_in_play_hypothesis);

        let is_truly_archetype_result = HypothesisResult::unimplemented();

        let result = and_result(archetype_in_play_result, is_truly_archetype_result);

        evaluator.create_return(result)
    }
}
