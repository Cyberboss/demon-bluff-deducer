use demon_bluff_gameplay_engine::{game_state::GameState, villager::Villager};
use log::Log;

use crate::{
    engine::{
        Depth, FitnessAndAction, Hypothesis, HypothesisBuilder, HypothesisRegistrar,
        HypothesisRepository, HypothesisResult, HypothesisReturn,
    },
    hypotheses::HypothesisType,
};

#[derive(Eq, PartialEq, Debug, Clone, Default)]
pub struct RevealingIsSafeHypothesisBuilder {}

#[derive(Debug)]
pub struct RevealingIsSafeHypothesis {}

impl HypothesisBuilder for RevealingIsSafeHypothesisBuilder {
    fn build<TLog>(
        self,
        game_state: &GameState,
        registrar: &mut HypothesisRegistrar<TLog>,
    ) -> HypothesisType
    where
        TLog: ::log::Log,
    {
        RevealingIsSafeHypothesis {}.into()
    }
}

impl Hypothesis for RevealingIsSafeHypothesis {
    fn describe(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "Revealing Villagers is Safe")
    }

    fn wip(&self) -> bool {
        true
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
        let mut all_hidden = true;
        for villager in game_state.villagers() {
            match villager {
                Villager::Hidden(_) => {}
                Villager::Active(_) | Villager::Confirmed(_) => {
                    all_hidden = false;
                    break;
                }
            }
        }

        // first reveal is always required, so call it safe even if it isn't
        if all_hidden {
            return repository.create_return(HypothesisResult::Conclusive(
                FitnessAndAction::certainty(None),
            ));
        }

        repository.create_return(HypothesisResult::unimplemented())
    }
}
