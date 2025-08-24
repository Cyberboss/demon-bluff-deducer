use demon_bluff_gameplay_engine::{
    affect::Affect,
    game_state::GameState,
    villager::{VillagerArchetype, VillagerIndex},
};
use log::Log;

use crate::{
    hypotheses::{HypothesisType, archetype_in_play::ArchetypeInPlayHypothesisBuilder},
    hypothesis::{
        Depth, FITNESS_UNKNOWN, FitnessAndAction, Hypothesis, HypothesisBuilder,
        HypothesisReference, HypothesisRegistrar, HypothesisRepository, HypothesisResult,
        HypothesisReturn, or_result,
    },
};

#[derive(Eq, PartialEq, Debug, Clone, Default)]
pub struct CorruptionInPlayHypothesisBuilder {}

/// Evaluates if corruption is in play
#[derive(Debug)]
pub struct CorruptionInPlayHypothesis {
    corrupting_archetype_hypotheses: Vec<HypothesisReference>,
}

impl HypothesisBuilder for CorruptionInPlayHypothesisBuilder {
    type HypothesisImpl = CorruptionInPlayHypothesis;

    fn build<TLog>(
        self,
        game_state: &GameState,
        registrar: &mut HypothesisRegistrar<TLog>,
    ) -> Self::HypothesisImpl
    where
        Self::HypothesisImpl: Hypothesis + 'static,
        HypothesisType: From<Self::HypothesisImpl>,
        TLog: ::log::Log,
    {
        let mut corrupting_archetype_hypotheses = Vec::new();
        for archetype in VillagerArchetype::iter() {
            if let Some(Affect::Corrupt(_)) =
                archetype.affects(game_state.total_villagers(), VillagerIndex(0))
            {
                corrupting_archetype_hypotheses
                    .push(registrar.register(ArchetypeInPlayHypothesisBuilder::new(archetype)));
            }
        }

        Self::HypothesisImpl {
            corrupting_archetype_hypotheses,
        }
    }
}

impl Hypothesis for CorruptionInPlayHypothesis {
    fn describe(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "Corruption in play")
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
        if self.corrupting_archetype_hypotheses.is_empty() {
            return repository
                .create_return(HypothesisResult::Conclusive(FitnessAndAction::impossible()));
        }

        let mut evaluator = repository.require_sub_evaluation(FITNESS_UNKNOWN);

        let mut sub_result = None;
        for sub_hypothesis in &self.corrupting_archetype_hypotheses {
            let new_sub_result = evaluator.sub_evaluate(sub_hypothesis);
            sub_result = Some(match sub_result {
                Some(old_sub_result) => or_result(old_sub_result, new_sub_result),
                None => new_sub_result,
            })
        }

        evaluator.create_return(sub_result.expect("Logic error in CorruptionInPlayHypothesis"))
    }
}
