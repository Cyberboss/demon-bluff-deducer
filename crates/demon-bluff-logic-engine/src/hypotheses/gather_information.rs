use std::collections::HashSet;

use demon_bluff_gameplay_engine::{game_state::GameState, villager::VillagerIndex};
use log::Log;

use crate::{
    hypotheses::{ability::AbilityHypothesis, reveal::RevealHypothesis},
    hypothesis::{
        Depth, FitnessAndAction, Hypothesis, HypothesisReference, HypothesisRegistrar,
        HypothesisRepository, HypothesisResult, HypothesisReturn, or_result,
    },
    player_action::{AbilityAttempt, PlayerAction},
};

#[derive(Eq, PartialEq, Debug)]
pub struct GatherInformationHypothesis {
    reveal_hypothesis: HypothesisReference,
    ability_hypothesis: HypothesisReference,
}

impl GatherInformationHypothesis {
    pub fn create<TLog>(
        game_state: &GameState,
        mut registrar: &mut HypothesisRegistrar<TLog>,
    ) -> HypothesisReference
    where
        TLog: Log,
    {
        let reveal_hypothesis = RevealHypothesis::create(game_state, &mut registrar);
        let ability_hypothesis = AbilityHypothesis::create(game_state, &mut registrar);
        registrar.register(Self {
            reveal_hypothesis,
            ability_hypothesis,
        })
    }
}

impl Hypothesis for GatherInformationHypothesis {
    fn describe(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "Gather Information")
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
        let mut evaluator = repository.require_sub_evaluation(0.0);

        let result = or_result(
            evaluator.sub_evaluate(&self.ability_hypothesis),
            evaluator.sub_evaluate(&self.reveal_hypothesis),
        );

        evaluator.create_return(result)
    }
}
