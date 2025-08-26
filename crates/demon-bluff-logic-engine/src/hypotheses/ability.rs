use demon_bluff_gameplay_engine::{game_state::GameState, villager::Villager};
use log::Log;

use crate::{
    engine::{
        Depth, Hypothesis, HypothesisBuilder, HypothesisEvaluation, HypothesisEvaluator,
        HypothesisFunctions, HypothesisReference, HypothesisRegistrar, HypothesisRepository,
        HypothesisResult, decide_result,
    },
    hypotheses::HypothesisType,
};

use super::{DesireType, HypothesisBuilderType, ability_index::AbilityIndexHypothesisBuilder};

#[derive(Eq, PartialEq, Debug, Default, Clone)]
pub struct AbilityHypothesisBuilder {}

impl HypothesisBuilder for AbilityHypothesisBuilder {
    fn build(
        self,
        game_state: &GameState,
        registrar: &mut impl HypothesisRegistrar<HypothesisBuilderType, DesireType>,
    ) -> HypothesisType {
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
    ) -> HypothesisEvaluation
    where
        TLog: Log,
    {
        if self.index_hypotheses.is_empty() {
            return repository.finalize(HypothesisResult::impossible());
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
        evaluator.finalize(result)
    }
}
