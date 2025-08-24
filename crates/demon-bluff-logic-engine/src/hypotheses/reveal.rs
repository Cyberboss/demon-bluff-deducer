use demon_bluff_gameplay_engine::{game_state::GameState, villager::Villager};
use log::Log;

use crate::{
    hypotheses::{HypothesisType, reveal_index::RevealIndexHypothesisBuilder},
    hypothesis::{
        Depth, FitnessAndAction, Hypothesis, HypothesisBuilder, HypothesisReference,
        HypothesisRegistrar, HypothesisRepository, HypothesisResult, HypothesisReturn, or_result,
    },
};

#[derive(Eq, PartialEq, Debug, Clone, Default)]
pub struct RevealHypothesisBuilder {}

#[derive(Debug)]
pub struct RevealHypothesis {
    revealable_hypotheses: Vec<HypothesisReference>,
}

impl HypothesisBuilder for RevealHypothesisBuilder {
    fn build<TLog>(
        self,
        game_state: &GameState,
        registrar: &mut HypothesisRegistrar<TLog>,
    ) -> HypothesisType
    where
        TLog: ::log::Log,
    {
        let mut revealable_hypotheses = Vec::new();
        game_state.iter_villagers(|villager_index, villager| match villager {
            Villager::Active(_) | Villager::Confirmed(_) => {}
            Villager::Hidden(hidden_villager) => {
                if !hidden_villager.cant_reveal() {
                    revealable_hypotheses.push(
                        registrar.register(RevealIndexHypothesisBuilder::new(villager_index)),
                    );
                }
            }
        });

        RevealHypothesis {
            revealable_hypotheses,
        }
        .into()
    }
}

impl Hypothesis for RevealHypothesis {
    fn describe(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "Reveal Decision")
    }

    fn evaluate<'a, 'b, TLog>(
        &'a mut self,
        _: &'a TLog,
        _: Depth,
        _: &'a GameState,
        repository: HypothesisRepository<'b, TLog>,
    ) -> HypothesisReturn
    where
        TLog: Log,
    {
        if self.revealable_hypotheses.is_empty() {
            return repository
                .create_return(HypothesisResult::Conclusive(FitnessAndAction::impossible()));
        }

        let mut evaluator = repository.require_sub_evaluation(0.0);
        let mut result = None;
        for reference in &self.revealable_hypotheses {
            let sub_evaluation = evaluator.sub_evaluate(reference);
            result = Some(match result {
                Some(existing_fitness) => or_result(sub_evaluation, existing_fitness),
                None => sub_evaluation,
            })
        }

        let result = result.expect("There should be a result after iterating");
        evaluator.create_return(result)
    }
}
