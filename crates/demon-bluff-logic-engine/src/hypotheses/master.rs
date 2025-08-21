use std::fmt::{Display, write};

use demon_bluff_gameplay_engine::game_state::{self, GameState};

use crate::{
    hypothesis::{
        EvaluationRequestFulfillment, EvaluationRequestResult, FitnessAndAction, Hypothesis,
        HypothesisContainer, HypothesisReference, HypothesisRegistrar, HypothesisRepository,
        HypothesisResult,
    },
    player_action::PlayerAction,
};

use super::reveal::RevealHypothesis;

#[derive(Debug)]
struct MasterHypothesis {
    reveal_hypothesis: HypothesisReference,
    execute_hypothesis: HypothesisReference,
    ability_hypothesis: HypothesisReference,
}

impl MasterHypothesis {
    pub fn create(
        game_state: &GameState,
        registrar: &mut HypothesisRegistrar,
    ) -> HypothesisReference {
        registrar.register(HypothesisContainer::new(Self {
            reveal_hypothesis: RevealHypothesis::create(game_state, registrar),
            execute_hypothesis: todo!(),
            ability_hypothesis: todo!(),
        }))
    }
}

impl Display for MasterHypothesis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Master Hypothesis")
    }
}

impl Hypothesis for MasterHypothesis {
    fn evaluate(
        &mut self,
        game_state: &GameState,
        repository: &mut HypothesisRepository,
    ) -> HypothesisResult {
        let evaluator_result = repository.request_sub_evaluation(0.0);
        let mut evaluator;
        let mut conclusive;
        match evaluator_result {
            EvaluationRequestResult::Approved(hypothesis_evaluator) => {
                evaluator = hypothesis_evaluator;
                conclusive = None;
            }
            EvaluationRequestResult::BreakCycle(hypothesis_evaluator) => {
                evaluator = hypothesis_evaluator;
                conclusive = Some(true);
            }
        }

        let mut best_fitness = None;
        let mut result = None;
        take_max_fitness(
            evaluator.sub_evaluate(&self.ability_hypothesis),
            &mut best_fitness,
            &mut conclusive,
            &mut result,
        );
        take_max_fitness(
            evaluator.sub_evaluate(&self.reveal_hypothesis),
            &mut best_fitness,
            &mut conclusive,
            &mut result,
        );
        take_max_fitness(
            evaluator.sub_evaluate(&self.execute_hypothesis),
            &mut best_fitness,
            &mut conclusive,
            &mut result,
        );

        result.expect("result should have been populated!")
    }
}

fn take_max_fitness(
    sub_hypothesis_result: HypothesisResult,
    best_fitness: &mut Option<FitnessAndAction>,
    conclusive: &mut Option<bool>,
    result: &mut Option<HypothesisResult>,
) {
    match sub_hypothesis_result {
        HypothesisResult::Weak(fitness_and_action) => todo!(),
        HypothesisResult::Conclusive(fitness_and_action) => todo!(),
    }
}
