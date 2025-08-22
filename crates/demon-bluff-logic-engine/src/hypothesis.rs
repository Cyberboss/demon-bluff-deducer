use std::{
    cell::{Ref, RefCell, RefMut},
    collections::{HashMap, HashSet, VecDeque},
    fmt::{Debug, Error, Formatter},
    result,
};

use demon_bluff_gameplay_engine::game_state::{self, GameState};
use force_graph::{DefaultNodeIdx, ForceGraph};
use log::{Log, error, info};
use thiserror::Error;

use crate::{
    hypotheses::{self, HypothesisType},
    player_action::PlayerAction,
};

/// A reference to a `Hypothesis` in the graph.
#[derive(Debug, PartialEq, Eq)]
pub struct HypothesisReference(usize);

/// A repository of hypotheses available to a single `Hypothesis` during evaluation.
pub struct HypothesisRepository<'a, TLog>
where
    TLog: Log,
{
    reference: HypothesisReference,
    evaluator: HypothesisEvaluator<'a, TLog>,
}

struct StackData<'a, TLog>
where
    TLog: Log,
{
    reference_stack: Vec<HypothesisReference>,
    log: &'a TLog,
    game_state: &'a GameState,
    hypotheses: &'a Vec<RefCell<HypothesisType>>,
    data: &'a RefCell<Vec<Option<HypothesisData>>>,
    graph_builder: Option<&'a RefCell<GraphBuilder>>,
    break_at: &'a Option<HypothesisReference>,
}

struct HypothesisInvocation<'a, TLog>
where
    TLog: Log,
{
    inner: StackData<'a, TLog>,
}

struct HypothesisData {
    dependencies: Vec<HypothesisReference>,
    last_evaluate: FitnessAndAction,
}

/// Used to evaluate sub-hypotheses via their `HypothesisReference`s.
pub struct HypothesisEvaluator<'a, TLog>
where
    TLog: Log,
{
    inner: StackData<'a, TLog>,
}

struct GraphBuilder {
    graph: ForceGraph<GraphNodeData>,
    node_map: HashMap<HypothesisReference, DefaultNodeIdx>,
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
#[derive(Clone)]
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
    fn describe(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error>;

    fn evaluate<TLog>(
        &mut self,
        log: &TLog,
        game_state: &::demon_bluff_gameplay_engine::game_state::GameState,
        repository: crate::hypothesis::HypothesisRepository<TLog>,
    ) -> crate::hypothesis::HypothesisReturn
    where
        TLog: ::log::Log;
}

pub struct HypothesisRegistrar {
    registrations: Vec<HypothesisType>,
}

#[derive(Error)]
pub enum EvaluationError {
    #[error("Evaluation could not determine an action to perform!")]
    ConclusiveNoAction,
}

impl<'a, TLog> StackData<'a, TLog>
where
    TLog: Log,
{
    fn new(
        game_state: &'a GameState,
        log: &'a TLog,
        hypotheses: &'a Vec<RefCell<HypothesisType>>,
        data: &'a RefCell<Vec<Option<HypothesisData>>>,
        break_at: &'a Option<HypothesisReference>,
        root_reference: &HypothesisReference,
        graph_builder: Option<&'a RefCell<GraphBuilder>>,
    ) -> Self {
        Self {
            reference_stack: vec![root_reference.clone()],
            log,
            game_state,
            hypotheses,
            data,
            break_at,
            graph_builder,
        }
    }

    fn share(&self) -> Self {
        let mut reference_stack = Vec::with_capacity(self.reference_stack.len());
        for mapped_reference in self
            .reference_stack
            .iter()
            .map(|reference| reference.clone())
        {
            reference_stack.push(mapped_reference);
        }
        Self {
            reference_stack,
            log: &self.log,
            game_state: &self.game_state,
            hypotheses: &self.hypotheses,
            data: &self.data,
            break_at: &self.break_at,
            graph_builder: self.graph_builder,
        }
    }

    fn push(&self, new_reference: HypothesisReference) -> Self {
        let mut reference_stack = Vec::with_capacity(self.reference_stack.len() + 1);
        for mapped_reference in self
            .reference_stack
            .iter()
            .map(|reference| reference.clone())
        {
            reference_stack.push(mapped_reference);
        }

        reference_stack.push(new_reference);

        Self {
            reference_stack,
            log: &self.log,
            game_state: &self.game_state,
            hypotheses: &self.hypotheses,
            data: &self.data,
            break_at: &self.break_at,
            graph_builder: self.graph_builder,
        }
    }
}

impl FitnessAndAction {
    pub fn fitness(&self) -> f64 {
        self.fitness
    }

    pub fn action(&self) -> &HashSet<PlayerAction> {
        &self.action
    }
}

impl<'a, TLog> HypothesisRepository<'a, TLog>
where
    TLog: Log,
{
    /// If a hypothesis has dependencies
    pub fn require_sub_evaluation(
        &mut self,
        initial_fitness: f64,
    ) -> &'a mut HypothesisEvaluator<TLog> {
        let mut data = self.evaluator.inner.data.borrow_mut();
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

impl<'a, TLog> HypothesisInvocation<'a, TLog>
where
    TLog: Log,
{
    fn new(stack_data: StackData<'a, TLog>) -> Self {
        Self { inner: stack_data }
    }

    fn enter(self) -> HypothesisResult {
        let reference = self
            .inner
            .reference_stack
            .last()
            .expect("There should be at least one reference in the stack!");
        let mut hypothesis = self.inner.hypotheses[reference.0].borrow_mut();
        let repository = HypothesisRepository {
            reference: reference.clone(),
            evaluator: HypothesisEvaluator {
                inner: self.inner.share(),
            },
        };

        let hypo_return = hypothesis.evaluate(self.inner.log, self.inner.game_state, repository);
        hypo_return.result
    }
}

impl<'a, TLog> HypothesisEvaluator<'a, TLog>
where
    TLog: Log,
{
    pub fn sub_evaluate(&mut self, hypothesis_reference: &HypothesisReference) -> HypothesisResult {
        let data = self.inner.data.borrow();
        let current_data = data[self
            .inner
            .reference_stack
            .last()
            .expect("There should be at least one reference in the stack")
            .0]
            .as_ref()
            .expect("How is hypothesis data not present if the reference can't be borrowed?");

        match self.inner.hypotheses[hypothesis_reference.0].try_borrow_mut() {
            Ok(next_reference) => {
                let invocation = HypothesisInvocation {
                    inner: self.inner.push(hypothesis_reference.clone()),
                };

                invocation.enter()
            }
            Err(_) => HypothesisResult::Pending(
                data[hypothesis_reference.0]
                    .as_ref()
                    .expect(
                        "How is hypothesis data not present if the reference can't be borrowed?",
                    )
                    .last_evaluate
                    .clone(),
            ),
        }
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

pub fn evaluate<TLog, F1, F2>(
    game_state: &GameState,
    hypothesis_factory: F1,
    log: &TLog,
    mut stepper: Option<F2>,
) -> Result<HashSet<PlayerAction>, EvaluationError>
where
    TLog: Log,
    F1: FnOnce(&GameState, &mut HypothesisRegistrar) -> HypothesisReference,
    F2: FnMut(&mut ForceGraph<GraphNodeData>),
{
    let mut registrar = HypothesisRegistrar {
        registrations: Vec::new(),
    };

    info!(logger: log, target: "evaluate", "Execute Hypothesis factory");
    let root: HypothesisReference = hypothesis_factory(game_state, &mut registrar);

    info!(logger: log, target: "evaluate", "Registered {} hypotheses. Root: {}", registrar.registrations.len(), registrar.registrations[root.0]);
    let hypotheses: Vec<RefCell<HypothesisType>> = registrar
        .registrations
        .into_iter()
        .map(|hypothesis| RefCell::new(hypothesis))
        .collect();

    let mut data = Vec::with_capacity(hypotheses.len());
    for _ in 0..hypotheses.len() {
        data.push(None);
    }

    let data = RefCell::new(data);
    let mut break_at = None;

    let mut iteration = 0;
    loop {
        iteration = iteration + 1;
        info!(logger: log, "Iteration: {}", iteration);

        let invocation = HypothesisInvocation::new(StackData::new(
            game_state,
            log,
            &hypotheses,
            &data,
            &break_at,
            &root,
            None,
        ));

        let result = invocation.enter();
        match result {
            HypothesisResult::Pending(fitness_and_action) => todo!(),
            HypothesisResult::Conclusive(fitness_and_action) => {
                if fitness_and_action.action.len() == 0 {
                    error!(logger: log, "Obtained conclusive result with no actions!");
                    return Err(EvaluationError::ConclusiveNoAction);
                }

                info!(logger: log, "Conclusive result obtained. Fitness: {}, Action(s): {}", fitness_and_action.fitness, fitness_and_action.action.len());

                if fitness_and_action.action.len() == 1 {
                    for action in &fitness_and_action.action {
                        info!(logger: log, "Conclusive action: {}", action);
                    }
                }

                return Ok(fitness_and_action.action);
            }
        }
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
