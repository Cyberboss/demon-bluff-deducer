use demon_bluff_gameplay_engine::{
    game_state::GameState,
    villager::{VillagerArchetype, VillagerIndex},
};
use log::Log;

use crate::{
    hypotheses::{
        archetype_in_play::{ArchetypeInPlayHypothesis, ArchetypeInPlayHypothesisBuilder},
        is_corrupt::{IsCorruptHypothesis, IsCorruptHypothesisBuilder},
        is_truthful::IsTruthfulHypothesis,
        negate::NegateHypothesis,
    },
    hypothesis::{
        Depth, FitnessAndAction, Hypothesis, HypothesisBuilder, HypothesisReference,
        HypothesisRegistrar, HypothesisRepository, HypothesisResult, HypothesisReturn,
    },
};

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct IsEvilHypothesisBuilder {
    index: VillagerIndex,
}

#[derive(Debug)]
pub struct IsEvilHypothesis {
    index: VillagerIndex,
    non_liars_in_play_hypotheses: Vec<HypothesisReference>,
    is_lying_hypothesis: HypothesisReference,
    is_corrupt_hypothesis: HypothesisReference,
}

impl HypothesisBuilder for IsEvilHypothesisBuilder {
    type HypothesisImpl = IsEvilHypothesis;

    fn build<TLog>(
        self,
        game_state: &GameState,
        registrar: &mut HypothesisRegistrar<TLog>,
    ) -> Self::HypothesisImpl
    where
        TLog: ::log::Log,
    {
        let mut non_liars_in_play_hypotheses = Vec::new();
        for archetype in VillagerArchetype::iter() {
            if archetype.is_evil() && !archetype.lies() {
                non_liars_in_play_hypotheses
                    .push(registrar.register(ArchetypeInPlayHypothesisBuilder::new(archetype)));
            }
        }

        let is_lying_hypothesis = registrar.register(NegateHypothesisBuilder::new(
            IsTruthfulHypothesisBuilder::new(self.index.clone()),
        ));
        let is_corrupt_hypothesis =
            registrar.register(IsCorruptHypothesisBuilder::new(self.index.clone()));

        Self::HypothesisImpl {
            index: self.index,
            non_liars_in_play_hypotheses,
            is_corrupt_hypothesis,
            is_lying_hypothesis,
        }
    }
}

impl Hypothesis for IsEvilHypothesis {
    fn describe(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{} is evil", self.index)
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
        repository.create_return(HypothesisResult::Conclusive(
            FitnessAndAction::unimplemented(),
        ))
    }
}
