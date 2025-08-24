use std::io::Write;
use std::{
    cell::RefCell,
    collections::{HashMap, HashSet, hash_map::Entry},
    fmt::{Debug, Display, Error, Formatter},
    fs::File,
};

use demon_bluff_gameplay_engine::game_state::{self, GameState};
use force_graph::{DefaultNodeIdx, ForceGraph};
use log::{Log, error, info, warn};
use serde::Serialize;

use crate::{
    PredictionError,
    desires::{self, DesireType},
    hypotheses::{self, HypothesisBuilderType, HypothesisType},
    player_action::PlayerAction,
};

pub const FITNESS_UNKNOWN: f64 = 0.5;

const ITERATIONS_BEFORE_GRAPH_ASSUMED_STABLE: u32 = 100;

const FITNESS_UNIMPLEMENTED: f64 = 0.000123456789;

/// A reference to a `Hypothesis` in the graph.
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct HypothesisReference(usize);

/// A repository of hypotheses available to a single `Hypothesis` during evaluation.
pub struct HypothesisRepository<'a, TLog>
where
    TLog: Log,
{
    set_desires: HashMap<DesireProducerReference, bool>,
    inner: StackData<'a, TLog>,
}

#[derive(Debug)]
struct StackData<'a, TLog>
where
    TLog: Log,
{
    reference_stack: Vec<HypothesisReference>,
    log: &'a TLog,
    game_state: &'a GameState,
    cycles: &'a RefCell<HashSet<Cycle>>,
    hypotheses: &'a Vec<RefCell<HypothesisType>>,
    previous_data: Option<&'a IterationData>,
    current_data: &'a RefCell<IterationData>,
    graph_builder: Option<&'a RefCell<GraphBuilder>>,
    break_at: &'a Option<HypothesisReference>,
    desire_definitions: &'a Vec<DesireDefinition>,
    dependencies: &'a DependencyData,
}

#[derive(Debug, PartialEq, Clone, Serialize)]
struct IterationData {
    desires: Vec<DesireData>,
    results: Vec<Option<HypothesisResult>>,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Cycle {
    order_from_root: Vec<HypothesisReference>,
}

#[derive(Debug)]
struct HypothesisGraph {
    root: HypothesisReference,
    hypotheses: Vec<HypothesisType>,
    dependencies: DependencyData,
    desires: Vec<DesireDefinition>,
}

/// A reference to a [`DesireType`] that a [`Hypothesis`] uses in its own [`HypothesisResult`] calculation
#[derive(Debug, PartialEq, Eq)]
pub struct DesireConsumerReference(usize);

/// A reference to a [`DesireType`] that a [`Hypothesis`] declares a desire for or not
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct DesireProducerReference(usize);

#[derive(Debug, PartialEq, Eq, Clone, Serialize)]
struct DesireData {
    pending: usize,
    desired: usize,
    undesired: usize,
}

#[derive(Debug)]
struct DesireDefinition {
    desire: DesireType,
    count: usize,
    used: bool,
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
    set_desires: HashMap<DesireProducerReference, bool>,
    inner: StackData<'a, TLog>,
}

struct GraphBuilder {
    graph: ForceGraph<GraphNodeData>,
    node_map: HashMap<HypothesisReference, DefaultNodeIdx>,
}

/// The return value of evaluating a single `Hypothesis`.
#[derive(Debug)]
pub struct HypothesisReturn {
    set_desires: HashMap<DesireProducerReference, bool>,
    result: HypothesisResult,
}

#[derive(Debug, PartialEq, Clone, Serialize)]
pub enum HypothesisResult {
    Pending(FitnessAndAction),
    Conclusive(FitnessAndAction),
}

/// Contains the fitness score of a given action set.
/// Fitness is the probability of how much a given `PlayerAction` will move the `GameState` towards a winning conclusion.
#[derive(Clone, Debug, PartialEq, Serialize)]
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
        depth: crate::engine::Depth,
        game_state: &::demon_bluff_gameplay_engine::game_state::GameState,
        repository: crate::engine::HypothesisRepository<TLog>,
    ) -> crate::engine::HypothesisReturn
    where
        TLog: ::log::Log;

    fn wip(&self) -> bool {
        false
    }
}

#[enum_delegate::register]
pub trait HypothesisBuilder {
    fn build<TLog>(
        self,
        game_state: &::demon_bluff_gameplay_engine::game_state::GameState,
        registrar: &mut crate::engine::HypothesisRegistrar<TLog>,
    ) -> HypothesisType
    where
        TLog: ::log::Log;
}

pub struct HypothesisRegistrar<'a, TLog>
where
    TLog: Log,
{
    log: &'a TLog,
    builders: Vec<HypothesisBuilderType>,
    desires: Vec<DesireType>,
    dependencies: Option<DependencyData>,
}

#[derive(Debug)]
struct DependencyData {
    desire_producers: Vec<Vec<DesireProducerReference>>,
    desire_consumers: Vec<Vec<DesireConsumerReference>>,
    hypotheses: Vec<Vec<HypothesisReference>>,
}

pub struct Depth {
    depth: usize,
    reference: HypothesisReference,
}

impl DesireData {
    fn total(&self) -> usize {
        self.undesired + self.pending + self.desired
    }
}

impl Display for DesireData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.desired, self.total())?;

        if self.pending > 0 {
            write!(f, " ({} Pending)", self.pending)
        } else {
            Ok(())
        }
    }
}

impl DesireProducerReference {
    pub fn clone(&self) -> Self {
        Self(self.0)
    }
}

impl Display for DesireProducerReference {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "D-{:05}", self.0 + 1)
    }
}

impl DesireConsumerReference {
    pub fn clone(&self) -> Self {
        Self(self.0)
    }
}

impl Display for DesireConsumerReference {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "D-{:05}", self.0 + 1)
    }
}

impl Display for DesireDefinition {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} ({} Producer(s)) {}",
            self.desire,
            self.count,
            if self.used { "" } else { " (UNUSED)" }
        )
    }
}

impl HypothesisReference {
    fn clone(&self) -> Self {
        Self(self.0)
    }

    pub fn unresolved() -> Self {
        Self(usize::MAX)
    }
}

impl Default for HypothesisReference {
    fn default() -> Self {
        Self::unresolved()
    }
}

impl Display for HypothesisReference {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "H-{:05}", self.0 + 1)
    }
}

impl Debug for GraphBuilder {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GraphBuilder")
            //.field("graph", &self.graph)
            .field("node_map", &self.node_map)
            .finish()
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
    pub fn unimplemented() -> Self {
        Self::Conclusive(FitnessAndAction::unimplemented())
    }

    pub fn impossible() -> Self {
        Self::Conclusive(FitnessAndAction::impossible())
    }

    pub fn map<F>(self, mut f: F) -> Self
    where
        F: FnMut(FitnessAndAction) -> FitnessAndAction,
    {
        match self {
            Self::Pending(fitness_and_action) => Self::Pending(f(fitness_and_action)),
            Self::Conclusive(fitness_and_action) => Self::Conclusive(f(fitness_and_action)),
        }
    }

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
        current_data: &'a RefCell<IterationData>,
        break_at: &'a Option<HypothesisReference>,
        root_reference: &HypothesisReference,
        graph_builder: Option<&'a RefCell<GraphBuilder>>,
        desire_definitions: &'a Vec<DesireDefinition>,
        dependencies: &'a DependencyData,
    ) -> Self {
        Self {
            reference_stack: vec![root_reference.clone()],
            log,
            game_state,
            previous_data,
            hypotheses,
            current_data,
            break_at,
            cycles,
            graph_builder,
            desire_definitions,
            dependencies,
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
            cycles: &self.cycles,
            desire_definitions: self.desire_definitions,
            dependencies: self.dependencies,
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
            cycles: &self.cycles,
            desire_definitions: self.desire_definitions,
            dependencies: self.dependencies,
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

impl Display for Depth {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for _ in 0..self.depth {
            write!(f, "  ")?
        }

        write!(f, "- [{}]", self.reference)
    }
}

impl FitnessAndAction {
    pub fn new(fitness: f64, action: Option<PlayerAction>) -> Self {
        let mut action_set = HashSet::with_capacity(1);
        if let Some(action) = action {
            action_set.insert(action);
        }

        Self {
            action: action_set,
            fitness,
        }
    }

    pub fn invert(mut self) -> Self {
        self.fitness = 1.0 - self.fitness;
        self
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

    pub fn certainty(action: Option<PlayerAction>) -> Self {
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
            set_desires: HashMap::new(),
            inner: self.inner.share(),
        };

        let hypo_return = hypothesis.evaluate(
            self.inner.log,
            self.inner.depth(),
            self.inner.game_state,
            repository,
        );

        info!(logger: self.inner.log, "{} Result: {}", self.inner.depth(), hypo_return.result);

        if let HypothesisResult::Conclusive(_) = &hypo_return.result {
            for producer_reference in &self.inner.dependencies.desire_producers[reference.0] {
                if !hypo_return.set_desires.contains_key(producer_reference) {
                    panic!(
                        "{}: {} was supposed to produce a result for {} before concluding but didn't!",
                        reference, hypothesis, producer_reference
                    )
                }
            }
        }

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
                if let Some(previous) = self.inner.previous_data
                    && let Some(_) = &previous.results[self.inner.current_reference().0]
                {
                } else {
                    info!(logger: self.inner.log, "{} Set initial fitness: {}",self.inner.depth(), initial_fitness);
                }
                data.results[self.inner.current_reference().0] =
                    Some(HypothesisResult::Pending(FitnessAndAction {
                        action: HashSet::new(),
                        fitness: initial_fitness,
                    }));
            }
        }

        HypothesisEvaluator {
            inner: self.inner,
            set_desires: self.set_desires,
        }
    }

    pub fn set_desire(&mut self, desire_reference: &DesireProducerReference, desired: bool) {
        let mut borrow = self.inner.current_data.borrow_mut();
        let data = &mut borrow.desires[desire_reference.0];

        let changed;
        match self.set_desires.entry(desire_reference.clone()) {
            Entry::Occupied(mut occupied_entry) => {
                if occupied_entry.insert(desired) == desired {
                    return;
                }

                changed = true;
            }
            Entry::Vacant(vacant_entry) => {
                vacant_entry.insert(desired);
                changed = false;
            }
        }

        if changed {
            if desired {
                data.undesired = data.undesired - 1;
            } else {
                data.desired = data.desired - 1;
            }
        } else {
            data.pending = data.pending - 1;
        }

        if desired {
            data.desired = data.desired + 1;
        } else {
            data.undesired = data.undesired + 1;
        }
    }

    pub fn desire_result(&self, desire_reference: &DesireConsumerReference) -> HypothesisResult {
        let definition = &self.inner.desire_definitions[desire_reference.0];
        let data = self
            .inner
            .previous_data
            .map(|previous_data| &previous_data.desires[desire_reference.0]);

        match data {
            Some(data) => {
                info!(logger: self.inner.log, "{} Read desire {} {} - {}", self.inner.depth(), desire_reference, definition.desire, data);
                let total = data.total();
                let fitness = FitnessAndAction::new(
                    if data.desired == 0 {
                        0.0 // stop divide by 0
                    } else {
                        (data.desired as f64) / (total as f64)
                    },
                    None,
                );
                if data.pending == 0 {
                    HypothesisResult::Conclusive(fitness)
                } else {
                    HypothesisResult::Pending(fitness)
                }
            }
            None => {
                info!(logger: self.inner.log, "{} Established desire dependency {}: {}", self.inner.depth(), desire_reference, definition);
                HypothesisResult::Pending(FitnessAndAction::new(FITNESS_UNKNOWN, None))
            }
        }
    }

    pub fn create_return(self, result: HypothesisResult) -> HypothesisReturn {
        HypothesisReturn {
            result,
            set_desires: self.set_desires,
        }
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

        let mut current_data = self.inner.current_data.borrow_mut();
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
                && let Some(HypothesisResult::Conclusive(previously_conclusive_result)) =
                    &previous_data.results[hypothesis_reference.0]
            {
                info!(logger: self.inner.log, "{} Skipping previously concluded hypothesis: {}", self.inner.depth(), hypothesis_reference);
                current_data.results[hypothesis_reference.0] = Some(HypothesisResult::Conclusive(
                    previously_conclusive_result.clone(),
                ));
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

    pub fn set_desire(&mut self, desire_reference: &DesireProducerReference, desired: bool) {
        let mut borrow = self.inner.current_data.borrow_mut();
        let data = &mut borrow.desires[desire_reference.0];

        let changed;
        match self.set_desires.entry(desire_reference.clone()) {
            Entry::Occupied(mut occupied_entry) => {
                if occupied_entry.insert(desired) == desired {
                    return;
                }

                changed = true;
            }
            Entry::Vacant(vacant_entry) => {
                vacant_entry.insert(desired);
                changed = false;
            }
        }

        if changed {
            if desired {
                data.undesired = data.undesired - 1;
            } else {
                data.desired = data.desired - 1;
            }
        } else {
            data.pending = data.pending - 1;
        }

        if desired {
            data.desired = data.desired + 1;
        } else {
            data.undesired = data.undesired + 1;
        }
    }

    pub fn desire_result(&self, desire_reference: &DesireProducerReference) -> HypothesisResult {
        let defintion = &self.inner.desire_definitions[desire_reference.0];
        let data = self
            .inner
            .previous_data
            .map(|previous_data| &previous_data.desires[desire_reference.0]);

        match data {
            Some(data) => {
                info!(logger: self.inner.log, "{} Read desire {} {} - {}", self.inner.depth(), desire_reference, defintion.desire, data);
                let total = data.total();
                let fitness = FitnessAndAction::new((data.desired as f64) / (total as f64), None);
                if data.pending == 0 || data.pending == total {
                    HypothesisResult::Conclusive(fitness)
                } else {
                    HypothesisResult::Pending(fitness)
                }
            }
            None => {
                info!(logger: self.inner.log, "{} Established desire {}: {}", self.inner.depth(), desire_reference, defintion);
                HypothesisResult::Pending(FitnessAndAction::new(FITNESS_UNKNOWN, None))
            }
        }
    }

    pub fn create_return(self, result: HypothesisResult) -> HypothesisReturn {
        HypothesisReturn {
            result,
            set_desires: self.set_desires,
        }
    }
}

impl<'a, TLog> HypothesisRegistrar<'a, TLog>
where
    TLog: Log,
{
    fn new(log: &'a TLog) -> Self {
        Self {
            log,
            builders: Vec::new(),
            dependencies: Some(DependencyData {
                hypotheses: Vec::new(),
                desire_consumers: Vec::new(),
                desire_producers: Vec::new(),
            }),
            desires: Vec::new(),
        }
    }

    /// Register a dependency of the currently building [`Hypothesis`]' [`HypothesisBuilder`] and get its [`HypothesisReference`].
    pub fn register<HypothesisBuilderImpl>(
        &mut self,
        builder: HypothesisBuilderImpl,
    ) -> HypothesisReference
    where
        HypothesisBuilderImpl: HypothesisBuilder,
        HypothesisBuilderType: From<HypothesisBuilderImpl>,
    {
        self.register_builder_type(builder.into())
    }

    pub fn register_builder_type(&mut self, builder: HypothesisBuilderType) -> HypothesisReference {
        let mut reference_option = None;
        for (index, existing_builder) in self.builders.iter().enumerate() {
            if builder == *existing_builder {
                reference_option = Some(HypothesisReference(index));
                break;
            }
        }

        let reference = match reference_option {
            Some(reference) => reference,
            None => {
                let reference = HypothesisReference(self.builders.len());
                self.builders.push(builder);
                reference
            }
        };

        if let Some(dependencies) = &mut self.dependencies {
            let dependencies_index = dependencies.hypotheses.len() - 1;
            dependencies.hypotheses[dependencies_index].push(reference.clone());
        }

        reference
    }

    fn register_desire_core(&mut self, desire: DesireType) -> usize {
        for (index, existing_desire) in self.desires.iter().enumerate() {
            if desire == *existing_desire {
                return index;
            }
        }

        let reference = self.desires.len();
        self.desires.push(desire);
        reference
    }

    pub fn register_desire_consumer(&mut self, desire: DesireType) -> DesireConsumerReference {
        let index = self.register_desire_core(desire.clone());
        let reference = DesireConsumerReference(index);

        if let Some(dependencies) = self.dependencies.as_mut() {
            let consumers = dependencies
                .desire_consumers
                .last_mut()
                .expect("Consumer entry should exist!");
            for existing_reference in consumers.iter() {
                if reference == *existing_reference {
                    return reference;
                }
            }

            consumers.push(reference.clone());
        }

        reference
    }

    pub fn register_desire_producer(&mut self, desire: DesireType) -> DesireProducerReference {
        let index = self.register_desire_core(desire.clone());
        let reference = DesireProducerReference(index);

        if let Some(dependencies) = self.dependencies.as_mut() {
            let producers = dependencies
                .desire_producers
                .last_mut()
                .expect("Producer entry should exist!");
            for existing_reference in producers.iter() {
                if reference == *existing_reference {
                    return reference;
                }
            }

            producers.push(reference.clone());
        }

        reference
    }

    fn run<HypothesisBuilderImpl>(
        mut self,
        game_state: &GameState,
        mut builder: HypothesisBuilderImpl,
    ) -> HypothesisGraph
    where
        HypothesisBuilderImpl: HypothesisBuilder,
        HypothesisBuilderType: From<HypothesisBuilderImpl>,
    {
        let mut current_reference = self.builders.len();
        let root_reference = HypothesisReference(current_reference);
        self.builders.push(builder.into());

        info!(logger: self.log, "Registering hypotheses builders");
        loop {
            let current_builder = self.builders[current_reference].clone();

            let dependency_data = self
                .dependencies
                .as_mut()
                .expect("Dependencies should exist");

            dependency_data.hypotheses.push(Vec::new());
            dependency_data.desire_consumers.push(Vec::new());
            dependency_data.desire_producers.push(Vec::new());

            // intentionally dropping the initially built hypotheis
            _ = current_builder.build(game_state, &mut self);

            current_reference = current_reference + 1;
            if current_reference == self.builders.len() {
                break;
            }
        }

        let dependencies = self
            .dependencies
            .take()
            .expect("Dependencies should still be here at this point");

        info!(logger: self.log, "Building hypotheses (Dependencies follow)");
        let mut hypotheses = Vec::with_capacity(current_reference);

        for (index, builder) in self.builders.clone().into_iter().enumerate() {
            let hypothesis = builder.build(game_state, &mut self).into();
            info!(logger: self.log, "{}: {}", HypothesisReference(index), hypothesis);
            for dependency in &dependencies.hypotheses[index] {
                info!(logger: self.log, "- {}", dependency);
            }

            hypotheses.push(hypothesis);
        }

        info!(logger: self.log, "Hypotheses built");

        info!(logger: self.log, "{} Desires:", self.desires.len());
        let mut desire_definitions = Vec::with_capacity(self.desires.len());
        for (index, desire) in self.desires.into_iter().enumerate() {
            let definition = DesireDefinition {
                desire: desire,
                count: dependencies
                    .desire_producers
                    .iter()
                    .filter(|producer_references| {
                        producer_references
                            .iter()
                            .any(|reference| reference.0 == index)
                    })
                    .count(),
                used: dependencies
                    .desire_consumers
                    .iter()
                    .any(|consumer_references| {
                        consumer_references
                            .iter()
                            .any(|reference| reference.0 == index)
                    }),
            };

            info!(logger: self.log, "- {}: {}", DesireProducerReference(index), definition);
            desire_definitions.push(definition);
        }

        HypothesisGraph {
            root: root_reference,
            hypotheses,
            dependencies,
            desires: desire_definitions,
        }
    }
}

pub fn evaluate<TBuilder, TLog, FGraph>(
    game_state: &GameState,
    initial_hypothesis_builder: TBuilder,
    log: &TLog,
    mut stepper: Option<FGraph>,
) -> Result<HashSet<PlayerAction>, PredictionError>
where
    TBuilder: HypothesisBuilder,
    HypothesisBuilderType: From<TBuilder>,
    TLog: Log,
    FGraph: FnMut(&mut ForceGraph<GraphNodeData>),
{
    let registrar = HypothesisRegistrar::new(log);

    info!(logger: log, target: "evaluate", "Evaluate dependencies");
    let graph = registrar.run(game_state, initial_hypothesis_builder.into());

    info!(logger: log, target: "evaluate", "Registered {} hypotheses. Root: {}", graph.hypotheses.len(), graph.hypotheses[graph.root.0]);

    let hypotheses: Vec<RefCell<HypothesisType>> = graph
        .hypotheses
        .into_iter()
        .map(|hypothesis| RefCell::new(hypothesis))
        .collect();

    let mut previous_results = None;

    let mut break_at = None;

    let mut iteration = 0;
    let mut stability_iteration = 0;
    loop {
        log.flush();
        iteration = iteration + 1;
        info!(logger: log, "Iteration: {}", iteration);

        let mut data = Vec::with_capacity(hypotheses.len());
        for _ in 0..hypotheses.len() {
            data.push(None);
        }

        let mut desires = Vec::with_capacity(graph.desires.len());
        for definition in &graph.desires {
            desires.push(DesireData {
                pending: definition.count,
                desired: 0,
                undesired: 0,
            });
        }

        let data = RefCell::new(IterationData {
            results: data,
            desires,
        });
        let cycles = RefCell::new(HashSet::new());
        let invocation = HypothesisInvocation::new(StackData::new(
            game_state,
            log,
            &hypotheses,
            &cycles,
            previous_results.as_ref(),
            &data,
            &break_at,
            &graph.root,
            None,
            &graph.desires,
            &graph.dependencies,
        ));

        let result = invocation.enter();

        break_at = None;

        match result {
            HypothesisResult::Pending(fitness_and_action) => {
                info!(logger: log, "Pending result. Fitness: {}", fitness_and_action);

                let data = data.borrow();
                if let Some(previous_results) = &previous_results {
                    let mut graph_stable = *previous_results == *data;

                    stability_iteration = stability_iteration + 1;
                    if !graph_stable {
                        /*
                        let mut f1 =
                            File::create("test1.json").expect("Failed to create before file");
                        let mut f2 =
                            File::create("test2.json").expect("Failed to create before file");

                        write!(
                            f1,
                            "{}",
                            serde_json::to_string_pretty(previous_results)
                                .expect("Serialization 1 failed")
                        )
                        .expect("File write 1 failed");
                        write!(
                            f2,
                            "{}",
                            serde_json::to_string_pretty(&*data).expect("Serialization 2 failed")
                        )
                        .expect("File write 1 failed");
                        log.flush();
                        */
                        if stability_iteration >= ITERATIONS_BEFORE_GRAPH_ASSUMED_STABLE {
                            warn!(logger: log, "Graph not stable after {} iterations, assuming stable enough for progression", ITERATIONS_BEFORE_GRAPH_ASSUMED_STABLE);
                            graph_stable = true;
                        }
                    } else {
                        info!(logger: log, "Graph stable")
                    }

                    if graph_stable {
                        stability_iteration = 0;

                        let cycles = cycles.borrow();
                        if cycles.len() > 0 {
                            warn!(logger: log, "We must break a cycle, of which there are {}", cycles.len());

                            let mut best_break_candidate =
                                None::<(&Cycle, &HypothesisReference, f64)>;

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
                                                (
                                                    previous_cycle,
                                                    previous_reference,
                                                    previous_fitness,
                                                )
                                            } else if fitness > previous_fitness {
                                                (cycle, reference, fitness)
                                            } else {
                                                // break shortest fittest candidate cycle first for simplicity
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

                            let (break_cycle, break_reference, break_fitness) =
                                best_break_candidate
                                    .expect("At least one break candidate should exist");
                            info!(logger: log, "Breaking cycle {} at {} which has a pending fitness value of {}", break_cycle, break_reference, break_fitness);

                            break_at = Some(break_reference.clone());
                        } else {
                            warn!(logger: log, "We must finalize an incomplete desire, of which there are {}", data.desires.iter().filter(|desire| desire.pending > 0).count());
                            todo!()
                        }
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
