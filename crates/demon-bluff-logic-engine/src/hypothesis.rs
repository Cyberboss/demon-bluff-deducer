use std::{
    cell::{Ref, RefCell, RefMut},
    collections::{HashMap, HashSet, VecDeque},
    fmt::{Debug, Error, Formatter},
    result,
};

use demon_bluff_gameplay_engine::game_state::{self, GameState};
use force_graph::ForceGraph;

use crate::{
    hypotheses::{self, HypothesisType},
    player_action::PlayerAction,
};

/// A reference to a `Hypothesis` in the graph.
#[derive(Debug, PartialEq, Eq)]
pub struct HypothesisReference(usize);

/// A repository of hypotheses available to a single `Hypothesis` during evaluation.
pub struct HypothesisRepository<'a> {
    reference: HypothesisReference,
    evaluator: HypothesisEvaluator<'a>,
    must_break: bool,
}

struct HypothesisInvocation {
    reference_stack: Vec<HypothesisReference>,
    hypotheses: Vec<RefCell<HypothesisType>>,
    data: RefCell<Vec<Option<HypothesisData>>>,
}

struct HypothesisData {
    dependencies: Vec<HypothesisReference>,
    last_evaluate: FitnessAndAction,
}

/// Used to evaluate sub-hypotheses via their `HypothesisReference`s.
pub struct HypothesisEvaluator<'a> {
    hypotheses: &'a Vec<RefCell<HypothesisType>>,
    data: &'a RefCell<Vec<Option<HypothesisData>>>,
}

/// The return value of evaluating a single `Hypothesis`.
pub struct HypothesisReturn {
    result: HypothesisResult,
}

/// The
pub enum HypothesisResult {
    Pending(FitnessAndAction),
    Conclusive(FitnessAndAction),
}

/// Contains the fitness score of a given action set.
/// Fitness is the probability of how much a given `PlayerAction` will move the `GameState` towards a winning conclusion.
pub struct FitnessAndAction {
    action: HashSet<PlayerAction>,
    fitness: f64,
}

pub struct GraphNodeData {
    description: String,
    current_fitness: Option<f64>,
}

#[enum_delegate::register]
pub trait Hypothesis {
    fn describe(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error>;

    fn evaluate(
        &mut self,
        game_state: &demon_bluff_gameplay_engine::game_state::GameState,
        repository: crate::hypothesis::HypothesisRepository,
    ) -> crate::hypothesis::HypothesisReturn;
}

pub struct HypothesisRegistrar {
    registrations: Vec<HypothesisType>,
}

impl FitnessAndAction {
    pub fn fitness(&self) -> f64 {
        self.fitness
    }

    pub fn action(&self) -> &HashSet<PlayerAction> {
        &self.action
    }
}

impl<'a> HypothesisRepository<'a> {
    /// If a hypothesis has dependencies
    pub fn require_sub_evaluation(&mut self, initial_fitness: f64) -> &'a mut HypothesisEvaluator {
        let mut data = self.evaluator.data.borrow_mut();
        match &data[self.reference.0] {
            Some(_) => {}
            None => {
                data[self.reference.0] = Some(HypothesisData {
                    dependencies: Vec::new(),
                    last_evaluate: FitnessAndAction {
                        action: HashSet::new(),
                        fitness: initial_fitness,
                    },
                });
            }
        }

        &mut self.evaluator
    }

    pub fn create_return(mut self, result: HypothesisResult) -> HypothesisReturn {
        HypothesisReturn { result }
    }
}

impl HypothesisInvocation {
    fn top_enter(mut self, game_state: &GameState, root: HypothesisReference) {
        let mut hypothesis = self.hypotheses[root.0].borrow_mut();
        let repository = HypothesisRepository {
            reference: root,
            must_break: false,
            evaluator: HypothesisEvaluator {
                hypotheses: &self.hypotheses,
                data: &self.data,
            },
        };

        let hypo_return = hypothesis.evaluate(game_state, repository);
        let result = hypo_return.result;
    }
}

impl<'a> HypothesisEvaluator<'a> {
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
        for (index, existing_container) in self.registrations.iter().enumerate() {
            if hypothesis == *existing_container {
                return HypothesisReference(index);
            }
        }

        self.registrations.push(hypothesis);
        HypothesisReference(self.registrations.len() - 1)
    }
}

impl HypothesisReference {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}

pub fn evaluate<F1, F2>(
    game_state: &GameState,
    hypothesis_factory: F1,
    mut stepper: Option<F2>,
) -> HashSet<PlayerAction>
where
    F1: FnOnce(&GameState, &mut HypothesisRegistrar) -> HypothesisReference,
    F2: FnMut(ForceGraph<GraphNodeData>),
{
    let mut registrar = HypothesisRegistrar {
        registrations: Vec::new(),
    };

    let root = hypothesis_factory(game_state, &mut registrar);

    let hypotheses: Vec<RefCell<HypothesisType>> = registrar
        .registrations
        .into_iter()
        .map(|hypothesis| RefCell::new(hypothesis))
        .collect();

    let data = Vec::with_capacity(hypotheses.len());
    for _ in 0..hypotheses.len() {
        data.push(None);
    }

    let data = RefCell::new(data);

    let mut invocation = HypothesisInvocation {
        reference_stack: vec![root.clone()],
        hypotheses,
        data,
    };

    invocation.enter(game_state, root);
    todo!();
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
