use demon_bluff_gameplay_engine::{
    game_state::GameState,
    villager::{VillagerArchetype, VillagerIndex},
};
use log::Log;

use crate::engine::{
    Depth, FITNESS_UNKNOWN, Hypothesis, HypothesisBuilder, HypothesisEvaluation,
    HypothesisEvaluator, HypothesisFunctions, HypothesisReference, HypothesisRegistrar,
    HypothesisRepository, or_result,
};

use super::{
    HypothesisBuilderType, HypothesisType, desires::DesireType, is_evil::IsEvilHypothesisBuilder,
    is_truly_archetype::IsTrulyArchetypeHypothesisBuilder,
};

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct AppearsEvilHypothesisBuilder {
    index: VillagerIndex,
}

#[derive(Debug)]
pub struct AppearsEvilHypothesis {
    index: VillagerIndex,
    is_evil_hypothesis: HypothesisReference,
    character_appears_evil_hypotheses: Vec<HypothesisReference>,
}

impl AppearsEvilHypothesisBuilder {
    pub fn new(index: VillagerIndex) -> Self {
        Self { index }
    }
}

impl HypothesisBuilder for AppearsEvilHypothesisBuilder {
    fn build(
        self,
        _: &GameState,
        registrar: &mut impl HypothesisRegistrar<HypothesisBuilderType, DesireType>,
    ) -> HypothesisType {
        let is_evil_hypothesis =
            registrar.register(IsEvilHypothesisBuilder::new(self.index.clone()));
        let mut character_appears_evil_hypotheses = Vec::new();
        for archetype in VillagerArchetype::iter() {
            if archetype.appears_evil() {
                character_appears_evil_hypotheses.push(registrar.register(
                    IsTrulyArchetypeHypothesisBuilder::new(archetype, self.index.clone()),
                ));
            }
        }

        AppearsEvilHypothesis {
            index: self.index,
            is_evil_hypothesis,
            character_appears_evil_hypotheses,
        }
        .into()
    }
}

impl Hypothesis for AppearsEvilHypothesis {
    fn describe(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{} appears evil", self.index)
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
        let mut evaluator = repository.require_sub_evaluation(FITNESS_UNKNOWN);

        let mut appears_evil_result = evaluator.sub_evaluate(&self.is_evil_hypothesis);

        for is_appears_evil_hypothesis in &self.character_appears_evil_hypotheses {
            let result = evaluator.sub_evaluate(is_appears_evil_hypothesis);
            appears_evil_result = or_result(appears_evil_result, result);
        }

        evaluator.finalize(appears_evil_result)
    }
}
