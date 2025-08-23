use std::{
    cell::{Ref, RefCell, RefMut},
    collections::{HashMap, HashSet, VecDeque},
    fmt::{Debug, Display, Error, Formatter, write},
    result,
};

use demon_bluff_gameplay_engine::game_state::{self, GameState};
use force_graph::{DefaultNodeIdx, ForceGraph};
use log::{Log, error, info, warn};
use thiserror::Error;

use crate::{
    PredictionError,
    hypotheses::{self, HypothesisType},
    player_action::PlayerAction,
};

const ITERATIONS_BEFORE_GRAPH_ASSUMED_STABLE: u32 = 1000;

const FITNESS_UNIMPLEMENTED: f64 = 0.123456789;

/// A reference to a `Hypothesis` in the graph.
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct HypothesisReference(usize);

/// A repository of hypotheses available to a single `Hypothesis` during evaluation.
pub struct HypothesisRepository<'a, TLog>
where
    TLog: Log,
{
    inner: StackData<'a, TLog>,
}

struct StackData<'a, TLog>
where
    TLog: Log,
{
    reference_stack: Vec<HypothesisReference>,
    log: &'a TLog,
    game_state: &'a GameState,
    cycles: &'a RefCell<HashSet<Cycle>>,
    hypotheses: &'a Vec<RefCell<HypothesisType>>,
    dependencies: &'a RefCell<Vec<HashSet<HypothesisReference>>>,
    previous_data: Option<&'a IterationData>,
    current_data: &'a RefCell<IterationData>,
    graph_builder: Option<&'a RefCell<GraphBuilder>>,
    break_at: &'a Option<HypothesisReference>,
}

#[derive(Debug, PartialEq, Clone)]
struct IterationData {
    results: Vec<Option<HypothesisResult>>,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Cycle {
    order_from_root: Vec<HypothesisReference>,
}

struct HypothesisInvocation<'a, TLog>
where
    TLog: Log,
{
    inner: StackData<'a, TLog>,
}

/// Used to evaluate sub-hypotheses via their `HypothesisReference`s.
#[derive(Debug)]
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
#[derive(Debug)]
pub struct HypothesisReturn {
    result: HypothesisResult,
}

#[derive(Debug, PartialEq, Clone)]
pub enum HypothesisResult {
    Pending(FitnessAndAction),
    Conclusive(FitnessAndAction),
}

/// Contains the fitness score of a given action set.
/// Fitness is the probability of how much a given `PlayerAction` will move the `GameState` towards a winning conclusion.
#[derive(Clone, Debug, PartialEq)]
pub struct FitnessAndAction {
    action: HashSet<PlayerAction>,
    fitness: f64,
}

#[derive(Debug)]
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
        depth: crate::hypothesis::Depth,
        game_state: &::demon_bluff_gameplay_engine::game_state::GameState,
        repository: crate::hypothesis::HypothesisRepository<TLog>,
    ) -> crate::hypothesis::HypothesisReturn
    where
        TLog: ::log::Log;
}

pub struct HypothesisRegistrar<'a, TLog>
where
    TLog: Log,
{
    log: &'a TLog,
    registrations: Vec<HypothesisType>,
}

pub struct Depth {
    depth: usize,
    reference: HypothesisReference,
}

impl Display for HypothesisReference {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "H-{:05}", self.0 + 1)
    }
}

impl Display for Cycle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut first = true;
        for reference in &self.order_from_root {
            if first {
                first = false;
            } else {
                write!(f, " -> ")?;
            }

            write!(f, "{}", reference)?;
        }

        Ok(())
    }
}

impl HypothesisResult {
    fn fitness_and_action(&self) -> &FitnessAndAction {
        match self {
            HypothesisResult::Pending(fitness_and_action)
            | HypothesisResult::Conclusive(fitness_and_action) => fitness_and_action,
        }
    }
}

impl Display for HypothesisResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            HypothesisResult::Pending(fitness_and_action) => {
                write!(f, "Pending: {}", fitness_and_action)
            }
            HypothesisResult::Conclusive(fitness_and_action) => {
                write!(f, "Conclusive: {}", fitness_and_action)
            }
        }
    }
}

impl<'a, TLog> StackData<'a, TLog>
where
    TLog: Log,
{
    fn new(
        game_state: &'a GameState,
        log: &'a TLog,
        hypotheses: &'a Vec<RefCell<HypothesisType>>,
        cycles: &'a RefCell<HashSet<Cycle>>,
        previous_data: Option<&'a IterationData>,
        dependencies: &'a RefCell<Vec<HashSet<HypothesisReference>>>,
        current_data: &'a RefCell<IterationData>,
        break_at: &'a Option<HypothesisReference>,
        root_reference: &HypothesisReference,
        graph_builder: Option<&'a RefCell<GraphBuilder>>,
    ) -> Self {
        Self {
            reference_stack: vec![root_reference.clone()],
            log,
            game_state,
            previous_data,
            dependencies,
            hypotheses,
            current_data,
            break_at,
            cycles,
            graph_builder,
        }
    }

    fn into_cycle(&self, locked_reference: &HypothesisReference) -> Cycle {
        let mut order_from_root = Vec::new();
        order_from_root.push(locked_reference.clone());
        let mut adding = false;
        for trace_reference in &self.reference_stack {
            if adding {
                order_from_root.push(trace_reference.clone());
            } else if trace_reference == locked_reference {
                adding = true;
            }
        }

        Cycle { order_from_root }
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
            current_data: &self.current_data,
            break_at: &self.break_at,
            graph_builder: self.graph_builder,
            previous_data: self.previous_data,
            dependencies: &self.dependencies,
            cycles: &self.cycles,
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
            current_data: &self.current_data,
            break_at: &self.break_at,
            graph_builder: self.graph_builder,
            previous_data: self.previous_data,
            dependencies: &self.dependencies,
            cycles: &self.cycles,
        }
    }

    fn current_reference(&self) -> &HypothesisReference {
        self.reference_stack
            .last()
            .expect("StackData should have at least one in stack!")
    }

    fn depth(&self) -> Depth {
        let reference = self.current_reference().clone();
        Depth {
            depth: self.reference_stack.len() - 1,
            reference,
        }
    }
}

impl<'a, TLog> Debug for StackData<'a, TLog>
where
    TLog: Log,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StackData")
            .field("reference_stack", &self.reference_stack)
            .field("game_state", &self.game_state)
            .field("hypotheses", &self.hypotheses)
            .field("current_data", &self.current_data)
            .field("previous_data", &self.previous_data)
            .field("break_at", &self.break_at)
            .finish()
    }
}

impl Display for Depth {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for _ in 0..self.depth {
            write!(f, "  ")?
        }

        write!(f, "- [{}]", self.reference)
    }
}

impl FitnessAndAction {
    pub fn new(fitness: f64, action: PlayerAction) -> Self {
        let mut action_set = HashSet::with_capacity(1);
        action_set.insert(action);
        Self {
            action: action_set,
            fitness,
        }
    }

    pub fn impossible() -> Self {
        Self {
            action: HashSet::new(),
            fitness: 0.0,
        }
    }

    pub fn unimplemented() -> Self {
        Self {
            action: HashSet::new(),
            fitness: FITNESS_UNIMPLEMENTED,
        }
    }

    pub fn certainty(action: PlayerAction) -> Self {
        Self::new(1.0, action)
    }

    pub fn is_certain(&self) -> bool {
        self.fitness == 1.0
    }

    pub fn fitness(&self) -> f64 {
        self.fitness
    }

    pub fn action(&self) -> &HashSet<PlayerAction> {
        &self.action
    }
}

impl Display for FitnessAndAction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.fitness == 0.0 {
            return write!(f, "Impossible");
        }

        if self.fitness == FITNESS_UNIMPLEMENTED {
            return write!(f, "UNIMPLEMENTED");
        }

        write!(f, "{:.2}% - ", self.fitness * 100.0)?;

        let length = self.action.len();
        if length == 0 {
            return write!(f, "(NO ACTION)");
        }

        for (index, action) in self.action.iter().enumerate() {
            if index != 0 {
                write!(f, ", ")?
            } else if index == length {
                write!(f, "or ")?
            }

            write!(f, "[{}]", action)?
        }

        Ok(())
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

        info!(logger: self.inner.log, "{} Entering: {}", self.inner.depth(), hypothesis);
        let repository = HypothesisRepository {
            inner: self.inner.share(),
        };

        let hypo_return = hypothesis.evaluate(
            self.inner.log,
            self.inner.depth(),
            self.inner.game_state,
            repository,
        );

        info!(logger: self.inner.log, "{} Result: {}", self.inner.depth(), hypo_return.result);

        let mut current_data = self.inner.current_data.borrow_mut();
        current_data.results[self.inner.current_reference().0] = Some(hypo_return.result.clone());

        hypo_return.result
    }
}

impl<'a, TLog> HypothesisRepository<'a, TLog>
where
    TLog: Log,
{
    /// If a hypothesis has dependencies
    pub fn require_sub_evaluation(self, initial_fitness: f64) -> HypothesisEvaluator<'a, TLog> {
        let mut data = self.inner.current_data.borrow_mut();
        match &data.results[self.inner.current_reference().0] {
            Some(_) => {}
            None => {
                info!(logger: self.inner.log, "{} Set initial fitness: {}",self.inner.depth(), initial_fitness);
                data.results[self.inner.current_reference().0] =
                    Some(HypothesisResult::Pending(FitnessAndAction {
                        action: HashSet::new(),
                        fitness: initial_fitness,
                    }));
            }
        }

        HypothesisEvaluator { inner: self.inner }
    }

    pub fn create_return(self, result: HypothesisResult) -> HypothesisReturn {
        HypothesisReturn { result }
    }
}

impl<'a, TLog> HypothesisEvaluator<'a, TLog>
where
    TLog: Log,
{
    pub fn sub_evaluate(&mut self, hypothesis_reference: &HypothesisReference) -> HypothesisResult {
        let current_reference = self
            .inner
            .reference_stack
            .last()
            .expect("There should be at least one reference in the stack");

        {
            let mut dependencies = self.inner.dependencies.borrow_mut();
            if dependencies[current_reference.0].insert(hypothesis_reference.clone()) {
                info!(
                    logger: self.inner.log,
                    "{} Established new hypothesis dependency: {}",
                    self.inner.depth(),
                    hypothesis_reference
                )
            }
        }

        let current_data = self.inner.current_data.borrow();
        let mut force_conclusive = false;
        if let Some(break_at) = self.inner.break_at
            && break_at == current_reference
        {
            info!(
                logger: self.inner.log,
                "{} Want to evaluate {} but we are breaking the cycle",
                self.inner.depth(),
                hypothesis_reference
            );

            force_conclusive = true;
        } else {
            if current_data.results[hypothesis_reference.0]
                .as_ref()
                .is_some()
            {
                info!(logger: self.inner.log, "{} Skipping re-evaluation of hypothesis: {}", self.inner.depth(), hypothesis_reference);
            } else if let Some(previous_data) = self.inner.previous_data
                && let Some(HypothesisResult::Conclusive(_)) =
                    &previous_data.results[hypothesis_reference.0]
            {
                info!(logger: self.inner.log, "{} Skipping previously concluded hypothesis: {}", self.inner.depth(), hypothesis_reference);
            } else {
                match self.inner.hypotheses[hypothesis_reference.0].try_borrow_mut() {
                    Ok(next_reference) => {
                        // Important or entering the invocation will BorrowError
                        drop(current_data);
                        drop(next_reference);

                        let invocation = HypothesisInvocation {
                            inner: self.inner.push(hypothesis_reference.clone()),
                        };

                        return invocation.enter();
                    }
                    Err(_) => {
                        info!(
                            logger: self.inner.log,
                            "{} Cycle detected when trying to evaluate reference {}",
                            self.inner.depth(),
                            hypothesis_reference
                        );

                        let cycle = self.inner.into_cycle(hypothesis_reference);

                        let mut cycles = self.inner.cycles.borrow_mut();
                        cycles.insert(cycle);
                    }
                }
            }
        }

        let relevant_iteration_data = current_data.results[hypothesis_reference.0]
            .as_ref()
            .unwrap_or_else(|| {
                self.inner
                    .previous_data
                    .expect("We shouldn't be using cached fitness data if none exists")
                    .results[hypothesis_reference.0]
                    .as_ref()
                    .expect("Fitness for cycle break didn't previously exist")
            });

        let mut last_evaluate = relevant_iteration_data.clone();

        if force_conclusive {
            last_evaluate = HypothesisResult::Conclusive(match last_evaluate {
                HypothesisResult::Pending(fitness_and_action)
                | HypothesisResult::Conclusive(fitness_and_action) => fitness_and_action,
            })
        }

        info!(
            logger: self.inner.log,
            "{} Using existing {} result: {}",
            self.inner.depth(),
            hypothesis_reference,
            last_evaluate
        );

        last_evaluate
    }

    pub fn create_return(self, result: HypothesisResult) -> HypothesisReturn {
        HypothesisReturn { result }
    }
}

impl<'a, TLog> HypothesisRegistrar<'a, TLog>
where
    TLog: Log,
{
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

        let reference = HypothesisReference(self.registrations.len());
        info!(logger: self.log, "Registered {}: {}", reference, hypothesis);

        self.registrations.push(hypothesis);
        reference
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
) -> Result<HashSet<PlayerAction>, PredictionError>
where
    TLog: Log,
    F1: FnOnce(&GameState, &mut HypothesisRegistrar<TLog>) -> HypothesisReference,
    F2: FnMut(&mut ForceGraph<GraphNodeData>),
{
    let mut registrar = HypothesisRegistrar {
        log,
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

    let mut previous_results = None;
    let mut dependencies = Vec::with_capacity(hypotheses.len());
    for _ in 0..hypotheses.len() {
        dependencies.push(HashSet::new());
    }

    let dependencies = RefCell::new(dependencies);

    let mut break_at = None;

    let mut iteration = 0;
    let mut stability_iteration = 0;
    loop {
        iteration = iteration + 1;
        info!(logger: log, "Iteration: {}", iteration);

        let mut data = Vec::with_capacity(hypotheses.len());
        for _ in 0..hypotheses.len() {
            data.push(None);
        }
        let data = RefCell::new(IterationData { results: data });
        let cycles = RefCell::new(HashSet::new());
        let invocation = HypothesisInvocation::new(StackData::new(
            game_state,
            log,
            &hypotheses,
            &cycles,
            previous_results.as_ref(),
            &dependencies,
            &data,
            &break_at,
            &root,
            None,
        ));

        let result = invocation.enter();

        break_at = None;

        match result {
            HypothesisResult::Pending(fitness_and_action) => {
                let data = data.borrow();
                if let Some(previous_results) = &previous_results {
                    let mut graph_stable = *previous_results == *data;

                    stability_iteration = stability_iteration + 1;
                    if !graph_stable
                        && stability_iteration >= ITERATIONS_BEFORE_GRAPH_ASSUMED_STABLE
                    {
                        warn!(logger: log, "Graph not stable after {} iterations, assuming stable enough for cycle breaking", ITERATIONS_BEFORE_GRAPH_ASSUMED_STABLE);
                        graph_stable = true;
                    }

                    if graph_stable {
                        stability_iteration = 0;

                        let cycles = cycles.borrow();
                        warn!(logger: log, "We must break a cycle, of which there are {}", cycles.len());

                        let mut best_break_candidate = None::<(&Cycle, &HypothesisReference, f64)>;

                        for cycle in cycles.iter() {
                            for reference in &cycle.order_from_root {
                                let fitness = data.results[reference.0]
                                    .as_ref()
                                    .expect("A hypothesis in a cycle should have SOME result")
                                    .fitness_and_action()
                                    .fitness;

                                best_break_candidate = Some(match best_break_candidate {
                                    Some((
                                        previous_cycle,
                                        previous_reference,
                                        previous_fitness,
                                    )) => {
                                        if previous_fitness > fitness {
                                            (previous_cycle, previous_reference, previous_fitness)
                                        } else if fitness > previous_fitness {
                                            (cycle, reference, fitness)
                                        } else {
                                            // break shortest candidate cycle first for simplicity
                                            if cycle.order_from_root.len()
                                                < previous_cycle.order_from_root.len()
                                            {
                                                (cycle, reference, fitness)
                                            } else {
                                                (
                                                    previous_cycle,
                                                    previous_reference,
                                                    previous_fitness,
                                                )
                                            }
                                        }
                                    }
                                    None => (cycle, reference, fitness),
                                });
                            }
                        }

                        let (break_cycle, break_reference, break_fitness) = best_break_candidate
                            .expect("At least one break candidate should exist");
                        info!(logger: log, "Breaking cycle {} at {} which has a pending fitness value of {}", break_cycle, break_reference, break_fitness);

                        break_at = Some(break_reference.clone());
                    }
                }

                previous_results = Some(data.clone());
            }
            HypothesisResult::Conclusive(fitness_and_action) => {
                if fitness_and_action.action.len() == 0 {
                    error!(logger: log, "Obtained conclusive result with no actions!");
                    return Err(PredictionError::ConclusiveNoAction);
                }

                info!(logger: log, "Conclusive result obtained. Fitness: {}", fitness_and_action);

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

pub fn and_result(lhs: HypothesisResult, rhs: HypothesisResult) -> HypothesisResult {
    let new_fitness_and_action;
    let must_be_pending;
    match lhs {
        HypothesisResult::Pending(fitness_and_action) => {
            must_be_pending = true;
            new_fitness_and_action = fitness_and_action
        }
        HypothesisResult::Conclusive(fitness_and_action) => {
            must_be_pending = false;
            new_fitness_and_action = fitness_and_action
        }
    }
    match rhs {
        HypothesisResult::Pending(current_fitness_and_action) => HypothesisResult::Pending(
            mult_fitness(current_fitness_and_action, new_fitness_and_action),
        ),
        HypothesisResult::Conclusive(current_fitness_and_action) => {
            let merged = mult_fitness(current_fitness_and_action, new_fitness_and_action);

            if must_be_pending {
                HypothesisResult::Pending(merged)
            } else {
                HypothesisResult::Conclusive(merged)
            }
        }
    }
}

pub fn or_result(lhs: HypothesisResult, rhs: HypothesisResult) -> HypothesisResult {
    let new_fitness_and_action;
    let must_be_pending;
    match lhs {
        HypothesisResult::Pending(fitness_and_action) => {
            must_be_pending = true;
            new_fitness_and_action = fitness_and_action
        }
        HypothesisResult::Conclusive(fitness_and_action) => {
            must_be_pending = false;
            new_fitness_and_action = fitness_and_action
        }
    }
    match rhs {
        HypothesisResult::Pending(current_fitness_and_action) => HypothesisResult::Pending(
            or_fitness_and_merge(current_fitness_and_action, new_fitness_and_action),
        ),
        HypothesisResult::Conclusive(current_fitness_and_action) => {
            let merged = or_fitness_and_merge(current_fitness_and_action, new_fitness_and_action);

            if must_be_pending {
                HypothesisResult::Pending(merged)
            } else {
                HypothesisResult::Conclusive(merged)
            }
        }
    }
}

pub fn merge_and_use_fittest_value(
    lhs: HypothesisResult,
    rhs: HypothesisResult,
) -> HypothesisResult {
    let new_fitness_and_action;
    let must_be_pending;
    match lhs {
        HypothesisResult::Pending(fitness_and_action) => {
            must_be_pending = true;
            new_fitness_and_action = fitness_and_action
        }
        HypothesisResult::Conclusive(fitness_and_action) => {
            must_be_pending = false;
            new_fitness_and_action = fitness_and_action
        }
    }
    match rhs {
        HypothesisResult::Pending(current_fitness_and_action) => HypothesisResult::Pending(
            max_fitness_and_merge(current_fitness_and_action, new_fitness_and_action),
        ),
        HypothesisResult::Conclusive(current_fitness_and_action) => {
            let merged = max_fitness_and_merge(current_fitness_and_action, new_fitness_and_action);

            if must_be_pending {
                HypothesisResult::Pending(merged)
            } else {
                HypothesisResult::Conclusive(merged)
            }
        }
    }
}

fn max_fitness_and_merge(mut lhs: FitnessAndAction, rhs: FitnessAndAction) -> FitnessAndAction {
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

fn mult_fitness(mut lhs: FitnessAndAction, rhs: FitnessAndAction) -> FitnessAndAction {
    for rh_action in rhs.action {
        lhs.action.insert(rh_action);
    }

    // P(A and B) = P(A) * P(B)
    lhs.fitness = lhs.fitness * rhs.fitness;
    lhs
}

fn or_fitness_and_merge(mut lhs: FitnessAndAction, rhs: FitnessAndAction) -> FitnessAndAction {
    for rh_action in rhs.action {
        lhs.action.insert(rh_action);
    }

    // P(A or B) = P(A) + P(B) - P(A and B)
    lhs.fitness = lhs.fitness + rhs.fitness - (lhs.fitness * rhs.fitness);
    lhs
}
