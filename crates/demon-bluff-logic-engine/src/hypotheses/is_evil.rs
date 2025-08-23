use demon_bluff_gameplay_engine::{
    game_state::GameState,
    villager::{Minion, VillagerArchetype, VillagerIndex},
};
use log::Log;

use crate::{
    hypotheses::{
        archetype_in_play::ArchetypeInPlayHypothesis, is_truthful::IsTruthfulHypothesis,
        negate::NegateHypothesis,
    },
    hypothesis::{
        Depth, FitnessAndAction, Hypothesis, HypothesisReference, HypothesisRegistrar,
        HypothesisRepository, HypothesisResult, HypothesisReturn,
    },
};

#[derive(Eq, PartialEq, Debug)]
pub struct IsEvilHypothesis {
    index: VillagerIndex,
    non_liars_in_play_hypotheses: Vec<HypothesisReference>,
    is_lying_hypothesis: HypothesisReference,
    is_corrupt_hypothesis: HypothesisReference,
}

impl IsEvilHypothesis {
    pub fn create<TLog>(
        game_state: &GameState,
        mut registrar: &mut HypothesisRegistrar<TLog>,
        index: VillagerIndex,
    ) -> HypothesisReference
    where
        TLog: Log,
    {
        let mut non_liars_in_play_hypotheses = Vec::new();
        for archetype in VillagerArchetype::iter() {
            if archetype.is_evil() && !archetype.lies() {
                non_liars_in_play_hypotheses.push(ArchetypeInPlayHypothesis::create(
                    game_state,
                    &mut registrar,
                    archetype,
                ));
            }
        }

        let is_truthful_hypothesis =
            IsTruthfulHypothesis::create(game_state, &mut registrar, index.clone());
        let is_lying_hypothesis =
            NegateHypothesis::create(game_state, &mut registrar, is_truthful_hypothesis);

        registrar.register(Self {
            index,
            non_liars_in_play_hypotheses,
            is_corrupt_hypothesis,
            is_lying_hypothesis,
        })
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
