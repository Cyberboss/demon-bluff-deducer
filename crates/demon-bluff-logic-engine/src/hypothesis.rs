use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    fmt::Debug,
};

use crate::{
    hypotheses::{self, HypothesisType},
    player_action::PlayerAction,
};

#[derive(Debug, PartialEq, Eq)]
pub struct HypothesisReference(usize);

pub struct HypothesisRepository {}

pub struct HypothesisEvaluator {}

pub enum EvaluationRequestResult {
    Approved(HypothesisEvaluator),
    BreakCycle(HypothesisEvaluator),
}

pub struct HypothesisReturn {
    result: HypothesisResult,
}

pub enum HypothesisResult {
    Pending(FitnessAndAction),
    Conclusive(FitnessAndAction),
}

pub struct FitnessAndAction {
    action: HashSet<PlayerAction>,
    fitness: f64,
}

pub struct HypothesisContainer {
    hypothesis: RefCell<HypothesisType>,
    last_evaluate: Option<FitnessAndAction>,
}

#[enum_delegate::register]
pub trait Hypothesis {
    fn evaluate(
        &mut self,
        game_state: &demon_bluff_gameplay_engine::game_state::GameState,
        repository: &mut crate::hypothesis::HypothesisRepository,
    ) -> crate::hypothesis::HypothesisReturn;
}

pub struct HypothesisRegistrar {
    hypotheses: Vec<HypothesisContainer>,
}

impl FitnessAndAction {
    pub fn fitness(&self) -> f64 {
        self.fitness
    }

    pub fn action(&self) -> &HashSet<PlayerAction> {
        &self.action
    }
}

impl HypothesisRepository {
    pub fn request_sub_evaluation(&mut self, current_fitness: f64) -> &mut EvaluationRequestResult {
        todo!()
    }

    pub fn require_sub_evaluation(&mut self, current_fitness: f64) -> &mut HypothesisEvaluator {
        let evaluator_result = self.request_sub_evaluation(current_fitness);
        match evaluator_result {
            EvaluationRequestResult::Approved(hypothesis_evaluator)
            | EvaluationRequestResult::BreakCycle(hypothesis_evaluator) => hypothesis_evaluator,
        }
    }

    pub fn create_return(&mut self, result: HypothesisResult) -> HypothesisReturn {
        todo!()
    }
}

impl HypothesisEvaluator {
    pub fn sub_evaluate(&mut self, hypothesis_reference: &HypothesisReference) -> HypothesisResult {
        todo!()
    }
}

impl HypothesisRegistrar {
    pub fn register<HypothesisImpl>(&mut self, hypothesis: HypothesisImpl) -> HypothesisReference
    where
        HypothesisImpl: Hypothesis + 'static,
        HypothesisType: From<HypothesisImpl>,
    {
        let hypothesis = hypothesis.into();
        for (index, existing_container) in self.hypotheses.iter().enumerate() {
            if hypothesis == *existing_container.hypothesis.borrow() {
                return HypothesisReference(index);
            }
        }

        let container = HypothesisContainer {
            hypothesis: RefCell::new(hypothesis),
            last_evaluate: None,
        };
        self.hypotheses.push(container);
        return HypothesisReference(self.hypotheses.len() - 1);
    }
}

pub fn fittest_result(
    sub_hypothesis_result: HypothesisResult,
    current_result: HypothesisResult,
) -> HypothesisResult {
    let new_fitness_and_action;
    let must_be_pending;
    match sub_hypothesis_result {
        HypothesisResult::Pending(fitness_and_action) => {
            must_be_pending = true;
            new_fitness_and_action = fitness_and_action
        }
        HypothesisResult::Conclusive(fitness_and_action) => {
            must_be_pending = false;
            new_fitness_and_action = fitness_and_action
        }
    }
    match current_result {
        HypothesisResult::Pending(current_fitness_and_action) => HypothesisResult::Pending(
            max_fitness(current_fitness_and_action, new_fitness_and_action),
        ),
        HypothesisResult::Conclusive(current_fitness_and_action) => {
            let merged = max_fitness(current_fitness_and_action, new_fitness_and_action);

            if must_be_pending {
                HypothesisResult::Pending(merged)
            } else {
                HypothesisResult::Conclusive(merged)
            }
        }
    }
}

fn max_fitness(mut lhs: FitnessAndAction, rhs: FitnessAndAction) -> FitnessAndAction {
    if lhs.fitness > rhs.fitness {
        lhs
    } else if rhs.fitness > lhs.fitness {
        rhs
    } else {
        for rh_action in rhs.action {
            lhs.action.insert(rh_action);
        }

        lhs
    }
}
