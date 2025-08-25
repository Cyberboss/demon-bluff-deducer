use std::collections::HashMap;

use demon_bluff_gameplay_engine::{
    game_state::GameState,
    villager::{VillagerArchetype, VillagerIndex},
};
use log::Log;

use crate::{
    engine::{
        Depth, FitnessAndAction, Hypothesis, HypothesisBuilder, HypothesisReference,
        HypothesisRegistrar, HypothesisRepository, HypothesisResult, HypothesisReturn, and_result,
        or_result,
    },
    hypotheses::{
        HypothesisType, archetype_in_play::ArchetypeInPlayHypothesisBuilder,
        is_corrupt::IsCorruptHypothesisBuilder, is_truthful::IsTruthfulHypothesisBuilder,
        negate::NegateHypothesisBuilder,
    },
};

use super::is_truly_archetype::IsTrulyArchetypeHypothesisBuilder;

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct IsEvilHypothesisBuilder {
    index: VillagerIndex,
}

#[derive(Debug)]
pub struct IsEvilHypothesis {
    index: VillagerIndex,
    is_non_liar_hypotheses: Vec<HypothesisReference>,
    is_lying_hypothesis: HypothesisReference,
    not_corrupt_hypothesis: HypothesisReference,
}

impl IsEvilHypothesisBuilder {
    pub fn new(index: VillagerIndex) -> Self {
        Self { index }
    }
}

impl HypothesisBuilder for IsEvilHypothesisBuilder {
    fn build<TLog>(self, _: &GameState, registrar: &mut HypothesisRegistrar<TLog>) -> HypothesisType
    where
        TLog: ::log::Log,
    {
        let mut is_non_liar_hypotheses = Vec::new();
        for archetype in VillagerArchetype::iter() {
            if archetype.is_evil() && !archetype.lies() {
                is_non_liar_hypotheses.push(registrar.register(
                    IsTrulyArchetypeHypothesisBuilder::new(archetype, self.index.clone()),
                ));
            }
        }

        let is_lying_hypothesis = registrar.register(NegateHypothesisBuilder::new(
            IsTruthfulHypothesisBuilder::new(self.index.clone()),
        ));
        let not_corrupt_hypothesis = registrar.register(NegateHypothesisBuilder::new(
            IsCorruptHypothesisBuilder::new(self.index.clone()),
        ));

        IsEvilHypothesis {
            index: self.index,
            is_non_liar_hypotheses,
            not_corrupt_hypothesis,
            is_lying_hypothesis,
        }
        .into()
    }
}

impl Hypothesis for IsEvilHypothesis {
    fn describe(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{} is evil", self.index)
    }

    fn evaluate<TLog>(
        &mut self,
        _: &TLog,
        _: Depth,
        game_state: &GameState,
        repository: HypothesisRepository<TLog>,
    ) -> HypothesisReturn
    where
        TLog: Log,
    {
        let initial_evils = game_state.draw_stats().demons() + game_state.draw_stats().minions();
        let mut evaluator = repository.require_sub_evaluation(
            (initial_evils as f64) / (game_state.draw_stats().total_villagers() as f64),
        );

        let not_corrupt_result = evaluator.sub_evaluate(&self.not_corrupt_hypothesis);
        let lying_result = evaluator.sub_evaluate(&self.is_lying_hypothesis);

        let regular_evil_result = and_result(not_corrupt_result, lying_result.clone());

        let mut is_a_truthful_evil_result = None;
        for sub_hypothesis in &self.is_non_liar_hypotheses {
            let is_archetype_result = evaluator.sub_evaluate(sub_hypothesis);

            is_a_truthful_evil_result = Some(match is_a_truthful_evil_result {
                Some(other_result) => or_result(other_result, is_archetype_result),
                None => is_archetype_result,
            })
        }

        let result = match is_a_truthful_evil_result {
            Some(is_a_truthful_evil_result) => {
                or_result(regular_evil_result, is_a_truthful_evil_result)
            }
            None => regular_evil_result,
        };

        evaluator.create_return(result)
    }
}
