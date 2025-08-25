use demon_bluff_gameplay_engine::{game_state::GameState, villager::Villager};
use log::Log;

use crate::{
    engine_old::{
        Depth, Hypothesis, HypothesisBuilder, HypothesisReference, HypothesisRegistrar,
        HypothesisRepository, HypothesisResult, HypothesisReturn, decide_result,
    },
    hypotheses::HypothesisType,
};

use super::ability_index::AbilityIndexHypothesisBuilder;

#[derive(Eq, PartialEq, Debug, Default, Clone)]
pub struct AbilityHypothesisBuilder {}

impl HypothesisBuilder for AbilityHypothesisBuilder {
    fn build<TLog>(
        self,
        game_state: &GameState,
        registrar: &mut HypothesisRegistrar<TLog>,
    ) -> HypothesisType
    where
        TLog: ::log::Log,
    {
        let mut index_hypotheses = Vec::new();
        game_state.iter_villagers(|index, villager| {
            let has_ability = match villager {
                Villager::Active(active_villager) => active_villager.instance().action_available(),
                Villager::Hidden(_) => false,
                Villager::Confirmed(confirmed_villager) => {
                    confirmed_villager.instance().action_available()
                }
            };

            if has_ability {
                index_hypotheses
                    .push(registrar.register(AbilityIndexHypothesisBuilder::new(index)));
            }
        });

        AbilityHypothesis { index_hypotheses }.into()
    }
}

#[derive(Debug)]
pub struct AbilityHypothesis {
    index_hypotheses: Vec<HypothesisReference>,
}

impl Hypothesis for AbilityHypothesis {
    fn describe(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "Ability Decision")
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
        if self.index_hypotheses.is_empty() {
            return repository.create_return(HypothesisResult::impossible());
        }

        let mut evaluator = repository.require_sub_evaluation(0.0);
        let mut result = None;
        for reference in &self.index_hypotheses {
            let sub_evaluation = evaluator.sub_evaluate(reference);
            result = Some(match result {
                Some(existing_fitness) => decide_result(sub_evaluation, existing_fitness),
                None => sub_evaluation,
            })
        }

        let result = result.expect("There should be a result after iterating");
        evaluator.create_return(result)
    }
}
