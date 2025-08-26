use demon_bluff_gameplay_engine::{game_state::GameState, villager::Villager};
use log::Log;

use crate::{
    engine::{
        Depth, FitnessAndAction, Hypothesis, HypothesisBuilder, HypothesisEvaluation,
        HypothesisReference, HypothesisRegistrar, HypothesisRepository, HypothesisResult,
        decide_result,
    },
    hypotheses::{HypothesisType, reveal_index::RevealIndexHypothesisBuilder},
};

#[derive(Eq, PartialEq, Debug, Clone, Default)]
pub struct RevealHypothesisBuilder {}

#[derive(Debug)]
pub struct RevealHypothesis {
    revealable_hypotheses: Vec<HypothesisReference>,
}

impl HypothesisBuilder for RevealHypothesisBuilder {
    fn build(
        self,
        game_state: &GameState,
        registrar: &mut impl HypothesisRegistrar<HypothesisBuilderType, DesireType>,
    ) -> HypothesisType {
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

    fn evaluate(
        &mut self,
        log: &impl Log,
        depth: Depth,
        game_state: &GameState,
        repository: impl HypothesisRepository,
    ) -> HypothesisEvaluation {
        if self.revealable_hypotheses.is_empty() {
            return repository
                .finalize(HypothesisResult::Conclusive(FitnessAndAction::impossible()));
        }

        let mut evaluator = repository.require_sub_evaluation(0.0);
        let mut result = None;
        for reference in &self.revealable_hypotheses {
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
